package br.com.mili.backend.data.dto;

import java.util.UUID;

public record UpdateOccurrenceStatusDto(UUID id,
                                        String status) {
}
