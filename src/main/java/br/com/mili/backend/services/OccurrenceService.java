package br.com.mili.backend.services;

import br.com.mili.backend.data.dto.CreateOccurrenceDto;
import br.com.mili.backend.data.dto.OccurrenceResponseDto;
import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
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
    public OccurrenceResponseDto createOccurrence(CreateOccurrenceDto dto) {
        var occ = new Occurrence();
        occ.setDescription(dto.description());
        var saved = repository.save(occ);

        var status = new OccurrenceStatus();
        status.setOccurrenceId(saved.getId());
        status.setStatus(OccurrenceStatusEnum.processing);
        status.setStatusDate(OffsetDateTime.now());
        occurrenceStatusRepository.save(status);

        return new OccurrenceResponseDto(saved.getId());
    }

}
