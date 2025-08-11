package br.com.mili.backend.repository;

import br.com.mili.backend.model.AppOccurrenceStatus;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.UUID;

@Repository
public interface AppOccurrenceStatusRepository extends JpaRepository<AppOccurrenceStatus, UUID> {
}
