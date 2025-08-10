package br.com.mili.backend.repository;

import br.com.mili.backend.model.Evidence;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.time.OffsetDateTime;
import java.util.Optional;
import java.util.UUID;

@Repository
public interface EvidenceRepository extends JpaRepository<br.com.mili.backend.model.Evidence, UUID> {
    @Query(value = """
    select ce.occurrence_id
    from camera_evidences ce
    join cameras c   on c.id = ce.camera_id
    join occurrences o on o.id = ce.occurrence_id
    where c.region = (select region from cameras where id = :cameraId)
      and ce.created_at >= :cutoff
      and o.finalized_at is null
    order by ce.created_at desc
    limit 1
    """, nativeQuery = true)
    Optional<UUID> findLatestOccurrenceForCameraRegionSince(
            @Param("cameraId") UUID cameraId,
            @Param("cutoff") OffsetDateTime cutoff);
}