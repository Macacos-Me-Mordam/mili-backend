package br.com.mili.backend.repository;

import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.model.Occurrence;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface OccurrenceRepository extends JpaRepository<Occurrence, UUID> {
    @Query("""
        select o from Occurrence o
        join OccurrenceStatus os on os.occurrenceId = o.id
        where os.id = (
          select os2.id from OccurrenceStatus os2
          where os2.occurrenceId = o.id
          order by os2.statusDate desc
          limit 1
        ) and os.status = :status
    """)
    List<Occurrence> findByStatus(OccurrenceStatusEnum status);
}
