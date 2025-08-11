package br.com.mili.backend.repository;

import br.com.mili.backend.data.enums.OccurrenceStatusEnum;
import br.com.mili.backend.model.AppOccurrence;
import br.com.mili.backend.model.Occurrence;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface AppOccurrenceRepository extends JpaRepository<AppOccurrence, UUID> {
    @Query("""
        select o from AppOccurrence o
        join AppOccurrenceStatus os on os.appOccurrenceId = o.id
        where os.id = (
          select os2.id from AppOccurrenceStatus os2
          where os2.appOccurrenceId = o.id
          order by os2.statusDate desc
          limit 1
        ) and os.status = :status
    """)
    List<AppOccurrence> findByStatus(OccurrenceStatusEnum status);
}
