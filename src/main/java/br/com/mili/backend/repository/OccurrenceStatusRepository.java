package br.com.mili.backend.repository;

import br.com.mili.backend.model.OccurrenceStatus;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface OccurrenceStatusRepository extends JpaRepository<OccurrenceStatus, UUID> {
    List<OccurrenceStatus> findByOccurrenceId(UUID occurrenceId);
}
