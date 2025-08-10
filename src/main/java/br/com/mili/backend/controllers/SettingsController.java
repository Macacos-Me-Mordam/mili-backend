package br.com.mili.backend.controllers;

import br.com.mili.backend.data.dto.EvidenceWindowDto;
import br.com.mili.backend.services.SettingsService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/settings")
public class SettingsController {

    @Autowired
    private  SettingsService service;

    @GetMapping("/evidence-window")
    public ResponseEntity<EvidenceWindowDto> getEvidenceWindow() {
        return ResponseEntity.ok(service.getEvidenceWindow());
    }

    @PutMapping("/evidence-window")
    public ResponseEntity<EvidenceWindowDto> setEvidenceWindow(@RequestBody EvidenceWindowDto body) {
        var res = service.updateEvidenceWindow(body.windowSeconds());
        return ResponseEntity.ok(res);
    }
}
