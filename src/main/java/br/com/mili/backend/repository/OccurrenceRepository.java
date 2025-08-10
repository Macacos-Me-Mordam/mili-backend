package br.com.mili.backend.repository;

import br.com.mili.backend.model.Occurrence;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface OccurrenceRepository extends JpaRepository<Occurrence, UUID> {
}
