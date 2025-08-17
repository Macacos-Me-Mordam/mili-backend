package br.com.mili.backend.service;

import br.com.mili.backend.data.dto.CreateOccurrenceDTO;
import br.com.mili.backend.data.dto.OccurrenceResponseDTO;
import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.exception.ResourceNotFoundException;
import br.com.mili.backend.model.OccurrenceStatus;
import br.com.mili.backend.model.Occurrence;
import br.com.mili.backend.repository.OccurrenceStatusRepository;
import br.com.mili.backend.repository.OccurrenceRepository;
import jakarta.transaction.Transactional;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.UUID;

@Service
public class OccurrenceService {

    private Logger logger = LoggerFactory.getLogger(UserService.class.getName());

    private OccurrenceRepository repository;
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
}
