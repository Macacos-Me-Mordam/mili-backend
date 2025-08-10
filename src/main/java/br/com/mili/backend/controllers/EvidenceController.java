package br.com.mili.backend.controllers;

import br.com.mili.backend.data.dto.CreateEvidenceDto;
import br.com.mili.backend.data.dto.CreateOccurrenceDto;
import br.com.mili.backend.data.dto.OccurrenceResponseDto;
import br.com.mili.backend.model.Evidence;
import br.com.mili.backend.repository.EvidenceRepository;
import br.com.mili.backend.services.OccurrenceService;
import br.com.mili.backend.services.SettingsService;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.bind.annotation.*;

import java.time.OffsetDateTime;

@RestController
@RequestMapping("/evidences")
public class EvidenceController {

    private final OccurrenceService service;
    private final EvidenceRepository evidenceRepo;
    private final SettingsService settingsService;

    public EvidenceController(
            OccurrenceService service,
            EvidenceRepository evidenceRepo,
            SettingsService settingsService
    ) {
        this.service = service;
        this.evidenceRepo = evidenceRepo;
        this.settingsService = settingsService;
    }

    @PostMapping(value = "/submit", consumes = MediaType.APPLICATION_JSON_VALUE)
    @Transactional
    public ResponseEntity<OccurrenceResponseDto> submitEvidence(@RequestBody CreateEvidenceDto payload) {
        var windowSeconds = settingsService.getEvidenceWindow().windowSeconds();
        var cutoff = OffsetDateTime.now().minusSeconds(windowSeconds);

        var occurrenceIdOpt = evidenceRepo.findLatestOccurrenceForCameraRegionSince(
                payload.cameraId(), cutoff);

        var occurrence = occurrenceIdOpt
                .map(id -> new OccurrenceResponseDto(id))
                .orElseGet(() -> service.createOccurrence(new CreateOccurrenceDto(payload.description())));

        var ev = new Evidence();
        ev.setFilePath(payload.filePath());
        ev.setCameraId(payload.cameraId());
        ev.setOccurrenceId(occurrence.id());
        evidenceRepo.save(ev);

        return ResponseEntity.ok(occurrence);
    }
}
