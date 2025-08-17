package br.com.mili.backend.controller;

import br.com.mili.backend.data.dto.EvidenceWindowDTO;
import br.com.mili.backend.service.SettingsService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/settings")
public class SettingsController {

    @Autowired
    private  SettingsService service;

    @GetMapping("/evidence-window")
    public ResponseEntity<EvidenceWindowDTO> getEvidenceWindow() {
        return ResponseEntity.ok(service.getEvidenceWindow());
    }

    @PutMapping("/evidence-window")
    public ResponseEntity<EvidenceWindowDTO> setEvidenceWindow(@RequestBody EvidenceWindowDTO body) {
        var res = service.updateEvidenceWindow(body.windowSeconds());
        return ResponseEntity.ok(res);
    }
}
