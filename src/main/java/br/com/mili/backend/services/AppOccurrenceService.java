package br.com.mili.backend.services;

import br.com.mili.backend.data.dto.CreateAppOccurrenceDto;
import br.com.mili.backend.data.dto.OccurrenceResponseDto;
import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.exception.ResourceNotFoundException;
import br.com.mili.backend.model.*;
import br.com.mili.backend.repository.AppOccurrenceRepository;
import br.com.mili.backend.repository.AppOccurrenceStatusRepository;
import jakarta.transaction.Transactional;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.UUID;

import static br.com.mili.backend.mapper.ObjectMapper.parseObject;

@Service
public class AppOccurrenceService {


    private Logger logger = LoggerFactory.getLogger(AppOccurrenceService.class.getName()); // Use a classe correta para o logger

    private AppOccurrenceRepository repository;
    private final AppOccurrenceStatusRepository appOccurrenceStatusRepository;

    @Autowired
    public AppOccurrenceService(AppOccurrenceStatusRepository appOccurrenceStatusRepository, AppOccurrenceRepository repository) {
        this.appOccurrenceStatusRepository = appOccurrenceStatusRepository;
        this.repository = repository;
    }

    @Transactional
    public OccurrenceResponseDto createOccurrence(CreateAppOccurrenceDto occurrence) {
        logger.info("Creating one app occurrence!");
        var entity = new AppOccurrence();
        entity.setDescription(occurrence.description());
        entity.setPhotoUrl(occurrence.photoUrl());
        entity.setAddress(occurrence.address());
        entity.setFrequency(occurrence.frequency());

        var savedEntity = repository.save(entity);

        var status = new AppOccurrenceStatus();
        status.setAppOccurrenceId(savedEntity.getId());
        status.setStatus(OccurrenceStatusEnum.processing);
        status.setStatusDate(OffsetDateTime.now());
        appOccurrenceStatusRepository.save(status);

        return new OccurrenceResponseDto(savedEntity.getId());
    }

    public List<AppOccurrence> getProcessingOccurrences() {
        logger.info("List processing Occurrences!");
        return repository.findByStatus(OccurrenceStatusEnum.processing);
    }

    public List<AppOccurrence> getResolvedOccurrences() {
        logger.info("List resolved Occurrences!");
        return repository.findByStatus(OccurrenceStatusEnum.resolved);
    }

    public List<AppOccurrence> getClosedOccurrences() {
        logger.info("List closed Occurrences!");
        return repository.findByStatus(OccurrenceStatusEnum.closed);
    }

    @Transactional
    public void updateOccurrenceStatus(UUID occurrenceId, OccurrenceStatusEnum newStatus) {
        logger.info("Updating status with OccurrenceId: {}", occurrenceId);
        repository.findById(occurrenceId)

                .orElseThrow(() -> new ResourceNotFoundException("Occurrence not found with id: " + occurrenceId));
        if (newStatus == OccurrenceStatusEnum.resolved || newStatus == OccurrenceStatusEnum.closed) {
            AppOccurrence occurrenceToFinalize = repository.findById(occurrenceId)
                    .orElseThrow(() -> new ResourceNotFoundException("Occurrence not found with id: " + occurrenceId));

            occurrenceToFinalize.setFinalizedAt(OffsetDateTime.now());
            repository.save(occurrenceToFinalize);
        }

        var status = new AppOccurrenceStatus();
        status.setAppOccurrenceId(occurrenceId);
        status.setStatus(newStatus);
        status.setStatusDate(OffsetDateTime.now());

        appOccurrenceStatusRepository.save(status);
    }

    public void delete(UUID id) {
        logger.info("Deleting one person!");
        AppOccurrence entity = repository.findById(id)
                .orElseThrow(() -> new ResourceNotFoundException("No records found for this ID"));
        repository.delete(entity);
    }
}
