package br.com.mili.backend.controller;

import br.com.mili.backend.data.dto.UpdateOccurrenceStatusDTO;
import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.model.Occurrence;
import br.com.mili.backend.service.OccurrenceService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/occurrences")
public class OccurenceController {

    @Autowired
    private OccurrenceService service;

    @GetMapping("/processing")
    public List<Occurrence> getProcessingOccurrences() {
        return service.getProcessingOccurrences();
    }

    @GetMapping("/resolved")
    public List<Occurrence> getResolvedOccurrences() {
        return service.getResolvedOccurrences();
    }

    @GetMapping("/closed")
    public List<Occurrence> getClosedOccurrences() {
        return service.getClosedOccurrences();
    }

    @PutMapping
    public ResponseEntity<Void> updateOccurrenceStatus(@RequestBody UpdateOccurrenceStatusDTO payload) {
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
