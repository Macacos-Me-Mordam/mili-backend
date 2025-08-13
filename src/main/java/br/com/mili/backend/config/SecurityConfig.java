package br.com.mili.backend.config;

import java.util.Arrays;
import java.util.List;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.HttpMethod;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.dao.DaoAuthenticationProvider;
import org.springframework.security.config.annotation.authentication.configuration.AuthenticationConfiguration;
import org.springframework.security.config.annotation.method.configuration.EnableMethodSecurity;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;
import org.springframework.web.cors.CorsConfiguration;
import org.springframework.web.cors.CorsConfigurationSource;
import org.springframework.web.cors.UrlBasedCorsConfigurationSource;

import br.com.mili.backend.security.jwt.JwtAuthFilter;

@Configuration
@EnableMethodSecurity
public class SecurityConfig {

    @Bean
    public DaoAuthenticationProvider authProvider(UserDetailsService uds, PasswordEncoder encoder) {
        var p = new DaoAuthenticationProvider();
        p.setUserDetailsService(uds);
        p.setPasswordEncoder(encoder);
        return p;
    }

    @Bean
    public AuthenticationManager authenticationManager(AuthenticationConfiguration cfg) throws Exception {
        return cfg.getAuthenticationManager();
    }

    // macacos-me-mordam/mili-backend/mili-backend-dev/src/main/java/br/com/mili/backend/config/SecurityConfig.java
// Arquivo: macacos-me-mordam/mili-backend/mili-backend-dev/src/main/java/br/com/mili/backend/config/SecurityConfig.java

@Bean
public SecurityFilterChain filterChain(HttpSecurity http,
        JwtAuthFilter jwtFilter,
        DaoAuthenticationProvider provider) throws Exception {
    http
            .csrf(csrf -> csrf.disable())
            .cors(cors -> {}) // usa o bean corsConfigurationSource()
            .sessionManagement(sm -> sm.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
            .authorizeHttpRequests(auth -> auth
                    .requestMatchers(HttpMethod.OPTIONS, "/**").permitAll() // Libera preflight
                    
                    // CORREÇÃO: Apenas login, logout e health são públicos. 
                    // A rota "/auth/profile" foi REMOVIDA desta linha.
                    .requestMatchers("/auth/login", "/auth/logout", "/health").permitAll() 
                    
                    // Agora, qualquer outra requisição (incluindo /auth/profile)
                    // exigirá autenticação, como deve ser.
                    .anyRequest().authenticated()) 
            .authenticationProvider(provider)
            .addFilterBefore(jwtFilter, UsernamePasswordAuthenticationFilter.class);

    return http.build();
}

    // ==== CORS ====
    // macacos-me-mordam/mili-backend/mili-backend-dev/src/main/java/br/com/mili/backend/config/SecurityConfig.java

// ... (resto da classe)

// ==== CORS ====
@Bean
public CorsConfigurationSource corsConfigurationSource() {
    CorsConfiguration cfg = new CorsConfiguration();

    // Defina as origens exatas que podem acessar sua API
    cfg.setAllowedOrigins(Arrays.asList(
            "http://localhost:3001",
            "http://127.0.0.1:3001"
    ));

    // Permite todos os métodos e cabeçalhos comuns
    cfg.setAllowedMethods(Arrays.asList("GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"));
    cfg.setAllowedHeaders(List.of("*"));
    
    // Permite o envio de credenciais (cookies, tokens)
    cfg.setAllowCredentials(true);

    UrlBasedCorsConfigurationSource source = new UrlBasedCorsConfigurationSource();
    source.registerCorsConfiguration("/**", cfg);
    return source;
}
}