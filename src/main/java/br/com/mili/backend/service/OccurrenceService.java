package br.com.mili.backend.service;

import br.com.mili.backend.data.dto.CreateOccurrenceDTO;
import br.com.mili.backend.data.dto.OccurrenceResponseDTO;
import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.exception.ResourceNotFoundException;
import br.com.mili.backend.model.Evidence;
import br.com.mili.backend.model.OccurrenceStatus;
import br.com.mili.backend.model.Occurrence;
import br.com.mili.backend.repository.OccurrenceStatusRepository;
import br.com.mili.backend.repository.OccurrenceRepository;
import com.lowagie.text.*;
import com.lowagie.text.Font;
import com.lowagie.text.pdf.PdfPCell;
import com.lowagie.text.pdf.PdfPTable;
import com.lowagie.text.pdf.PdfWriter;
import jakarta.transaction.Transactional;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.awt.*;
import java.io.ByteArrayOutputStream;
import java.time.OffsetDateTime;
import java.time.format.DateTimeFormatter;
import java.util.List;
import java.util.UUID;

@Service
public class OccurrenceService {

    private final Logger logger = LoggerFactory.getLogger(OccurrenceService.class.getName());

    private final OccurrenceRepository repository;
    private final OccurrenceStatusRepository occurrenceStatusRepository;

    @Autowired
    public OccurrenceService(OccurrenceStatusRepository occurrenceStatusRepository, OccurrenceRepository repository) {
        this.occurrenceStatusRepository = occurrenceStatusRepository;
        this.repository = repository;
    }

    @Transactional
    public OccurrenceResponseDTO createOccurrence(CreateOccurrenceDTO dto) {
        logger.info("Creating one app occurrence!");
        var occ = new Occurrence();
        occ.setDescription(dto.description());
        var saved = repository.save(occ);

        var status = new OccurrenceStatus();
        status.setOccurrenceId(saved.getId());
        status.setStatus(OccurrenceStatusEnum.processing);
        status.setStatusDate(OffsetDateTime.now());
        occurrenceStatusRepository.save(status);

        return new OccurrenceResponseDTO(saved.getId());
    }

    public List<Occurrence> getProcessingOccurrences() {
        logger.info("List processing Occurrences!");
        return repository.findByStatus(OccurrenceStatusEnum.processing);
    }

    public List<Occurrence> getResolvedOccurrences() {
        logger.info("List resolved Occurrences!");
        return repository.findByStatus(OccurrenceStatusEnum.resolved);
    }

    public List<Occurrence> getClosedOccurrences() {
        logger.info("List closed Occurrences!");
        return repository.findByStatus(OccurrenceStatusEnum.closed);
    }

    @Transactional
    public void updateOccurrenceStatus(UUID occurrenceId, OccurrenceStatusEnum newStatus) {
        logger.info("Updating status with OccurrenceId: {}", occurrenceId);
        repository.findById(occurrenceId)
                .orElseThrow(() -> new ResourceNotFoundException("Occurrence not found with id: " + occurrenceId));

        if (newStatus == OccurrenceStatusEnum.resolved || newStatus == OccurrenceStatusEnum.closed) {
            Occurrence occurrenceToFinalize = repository.findById(occurrenceId)
                    .orElseThrow(() -> new ResourceNotFoundException("Occurrence not found with id: " + occurrenceId));

            occurrenceToFinalize.setFinalizedAt(OffsetDateTime.now());
            repository.save(occurrenceToFinalize);
        }

        var status = new OccurrenceStatus();
        status.setOccurrenceId(occurrenceId);
        status.setStatus(newStatus);
        status.setStatusDate(OffsetDateTime.now());

        occurrenceStatusRepository.save(status);
    }

    public void delete(UUID id) {
        logger.info("Deleting one person!");
        Occurrence entity = repository.findById(id)
                .orElseThrow(() -> new ResourceNotFoundException("No records found for this ID"));
        repository.delete(entity);
    }

    public byte[] generateOccurrenceProof(UUID id) {
        Occurrence entity = repository.findById(id)
                .orElseThrow(() -> new ResourceNotFoundException("No records found for this ID"));

        List<OccurrenceStatus> statusHistory = occurrenceStatusRepository.findByOccurrenceId(id);

        if (statusHistory != null && !statusHistory.isEmpty()) {
            if (statusHistory.getLast().getStatus() == OccurrenceStatusEnum.processing) {
                throw new IllegalArgumentException("You cannot generate proof of an unfinished occurrence.");
            }
        }

        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        Document document = new Document();

        try {
            PdfWriter.getInstance(document, baos);
            document.open();

            Font titleFont = FontFactory.getFont(FontFactory.HELVETICA_BOLD, 18, Font.NORMAL, Color.BLACK);
            Font subtitleFont = FontFactory.getFont(FontFactory.HELVETICA_BOLD, 14, Font.NORMAL, Color.DARK_GRAY);
            Font boldFont = FontFactory.getFont(FontFactory.HELVETICA_BOLD, 12);
            Font normalFont = FontFactory.getFont(FontFactory.HELVETICA, 12);
            DateTimeFormatter formatter = DateTimeFormatter.ofPattern("dd/MM/yyyy HH:mm:ss");

            document.add(new Paragraph("Comprovante", titleFont));
            document.add(new Paragraph(" ")); // Linha em branco

            document.add(new Paragraph("DETALHES DA OCORRÊNCIA", subtitleFont));
            document.add(new Paragraph(" "));

            document.add(new Paragraph("ID da Ocorrência: " + entity.getId().toString(), normalFont));
            document.add(new Paragraph("Descrição: " + entity.getDescription(), normalFont));
            document.add(new Paragraph("Data de Criação: " + entity.getCreatedAt().format(formatter), normalFont));
            if (entity.getFinalizedAt() != null) {
                document.add(new Paragraph("Data de Finalização: " + entity.getFinalizedAt().format(formatter), normalFont));
            }
            if (statusHistory != null) {
                document.add(new Paragraph("Status: " + statusHistory.getLast().getStatus(), normalFont));
            }
            document.add(new Paragraph(" "));

            if (entity.getEvidences() != null && !entity.getEvidences().isEmpty()) {
                document.add(new Paragraph("EVIDÊNCIAS REGISTRADAS", subtitleFont));
                document.add(new Paragraph(" "));

                PdfPTable evidenceTable = new PdfPTable(2);
                evidenceTable.setWidthPercentage(100);

                evidenceTable.addCell(new PdfPCell(new Paragraph("Caminho do Arquivo", boldFont)));
                evidenceTable.addCell(new PdfPCell(new Paragraph("Data de Registro", boldFont)));

                for (Evidence evidence : entity.getEvidences()) {
                    evidenceTable.addCell(new PdfPCell(new Paragraph(evidence.getFilePath(), normalFont)));
                    evidenceTable.addCell(new PdfPCell(new Paragraph(evidence.getCreatedAt().format(formatter), normalFont)));
                }
                document.add(evidenceTable);
            }

            document.close();
        } catch (DocumentException e) {
            logger.error("Erro ao gerar PDF para a ocorrência: {}", e.getMessage());
        }

        return baos.toByteArray();
    }
}
