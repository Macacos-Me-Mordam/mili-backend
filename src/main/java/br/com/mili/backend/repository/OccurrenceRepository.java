package br.com.mili.backend.repository;

import br.com.mili.backend.model.Occurrence;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.UUID;

@Repository
public interface OccurrenceRepository extends JpaRepository<Occurrence, UUID> {
}
