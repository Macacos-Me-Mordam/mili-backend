package br.com.mili.backend.services;

import br.com.mili.backend.data.dto.EvidenceWindowDto;
import br.com.mili.backend.model.Settings;
import br.com.mili.backend.repository.SettingsRepository;
import jakarta.transaction.Transactional;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

@Service
public class SettingsService {

    private static final String KEY_WINDOW = "evidence.window.seconds";

    private SettingsRepository repository;

    @Value("${app.evidence.window-seconds-default:60}")
    private long defaultWindowSeconds;

    public SettingsService(SettingsRepository repository) {
        this.repository = repository;
    }

    public EvidenceWindowDto getEvidenceWindow() {
        var opt = repository.findById(KEY_WINDOW);
        long seconds = opt.map(s -> parseLongOrDefault(s.getValue(), defaultWindowSeconds))
                .orElse(defaultWindowSeconds);
        return new EvidenceWindowDto(seconds);
    }

    @Transactional
    public EvidenceWindowDto updateEvidenceWindow(long seconds) {
        if (seconds <= 0) throw new IllegalArgumentException("windowSeconds deve ser > 0");
        var setting = repository.findById(KEY_WINDOW).orElse(new Settings(KEY_WINDOW, String.valueOf(seconds)));
        setting.setValue(String.valueOf(seconds));
        repository.save(setting);
        return new EvidenceWindowDto(seconds);
    }

    private long parseLongOrDefault(String val, long def) {
        try { return Long.parseLong(val); } catch (Exception e) { return def; }
    }
}
