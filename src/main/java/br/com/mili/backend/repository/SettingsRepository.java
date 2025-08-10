package br.com.mili.backend.repository;

import br.com.mili.backend.model.Settings;
import org.springframework.data.jpa.repository.JpaRepository;

public interface SettingsRepository extends JpaRepository<Settings, String> {
}
