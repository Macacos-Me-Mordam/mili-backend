package br.com.mili.backend.controllers;

import br.com.mili.backend.data.dto.CreateAppOccurrenceDto;
import br.com.mili.backend.data.dto.OccurrenceResponseDto;
import br.com.mili.backend.data.dto.UpdateOccurrenceStatusDto;
import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.model.AppOccurrence;
import br.com.mili.backend.services.AppOccurrenceService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/app-occurrence")
public class AppOccurrenceController {

    @Autowired
    private AppOccurrenceService service;

    @PostMapping
    public OccurrenceResponseDto createOccurrence(@RequestBody CreateAppOccurrenceDto payload) {
        var response = service.createOccurrence(payload);
        return response;
    }

    @GetMapping("/processing")
    public List<AppOccurrence> getProcessingOccurrences() {
        return service.getProcessingOccurrences();
    }


    @GetMapping("/resolved")
    public List<AppOccurrence> getResolvedOccurrences() {
        return service.getResolvedOccurrences();
    }

    @GetMapping("/closed")
    public List<AppOccurrence> getClosedOccurrences() {
        return service.getClosedOccurrences();
    }

    @PutMapping("/status")
    public ResponseEntity<Void> updateOccurrenceStatus(@RequestBody UpdateOccurrenceStatusDto payload) {
        var statusEnum = OccurrenceStatusEnum.valueOf(payload.status());
        service.updateOccurrenceStatus(payload.id(), statusEnum);
        return ResponseEntity.ok().build();
    }

    @DeleteMapping(value = "/{id}")
    public ResponseEntity<?> delete(@PathVariable("id") UUID id) {
        service.delete(id);
        return ResponseEntity.noContent().build();
    }
}
