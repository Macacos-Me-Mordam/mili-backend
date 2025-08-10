package br.com.mili.backend.controllers;

import br.com.mili.backend.data.dto.LoginRequestDto;
import br.com.mili.backend.data.dto.LoginResponseDto;
import br.com.mili.backend.security.jwt.JwtService;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.http.ResponseCookie;
import org.springframework.http.ResponseEntity;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

import java.util.Map;

@RestController
@RequestMapping("/auth")
public class AuthController {

    private final AuthenticationManager authManager;
    private final JwtService jwtService;
    private final String cookieName;
    private final String cookieDomain;
    private final boolean cookieSecure;
    private final String cookieSameSite;
    private final long expiration;

    public AuthController(AuthenticationManager authManager,
                          JwtService jwtService,
                          @Value("${app.jwt.cookie-name}") String cookieName,
                          @Value("${app.jwt.cookie-domain}") String cookieDomain,
                          @Value("${app.jwt.cookie-secure}") boolean cookieSecure,
                          @Value("${app.jwt.cookie-samesite}") String cookieSameSite,
                          @Value("${app.jwt.expiration}") long expiration) {
        this.authManager = authManager;
        this.jwtService = jwtService;
        this.cookieName = cookieName;
        this.cookieDomain = cookieDomain;
        this.cookieSecure = cookieSecure;
        this.cookieSameSite = cookieSameSite;
        this.expiration = expiration;
    }

    @PostMapping("/login")
    public ResponseEntity<LoginResponseDto> login(@RequestBody LoginRequestDto req) {
        Authentication auth = authManager.authenticate(
                new UsernamePasswordAuthenticationToken(req.email(), req.password())
        );

        var principal = (UserDetails) auth.getPrincipal();
        String token = jwtService.generate(principal.getUsername(), Map.of("roles", principal.getAuthorities()));

        ResponseCookie cookie = ResponseCookie.from(cookieName, token)
                .httpOnly(true)
                .secure(cookieSecure)
                .domain(cookieDomain)
                .path("/")
                .sameSite(cookieSameSite)
                .maxAge(expiration)
                .build();

        return ResponseEntity.ok()
                .header("Set-Cookie", cookie.toString())
                .body(new LoginResponseDto("Authenticated!"));
    }

    @PostMapping("/logout")
    public ResponseEntity<LoginResponseDto> logout() {
        ResponseCookie cookie = ResponseCookie.from(cookieName, "")
                .httpOnly(true)
                .secure(cookieSecure)
                .domain(cookieDomain)
                .path("/")
                .sameSite(cookieSameSite)
                .maxAge(0)
                .build();

        return ResponseEntity.ok()
                .header("Set-Cookie", cookie.toString())
                .body(new LoginResponseDto("logged out"));
    }


    @GetMapping("/profile")
    public Map<String, Object> profile(Authentication authentication) {
        return Map.of("name", authentication.getName(), "authorities", authentication.getAuthorities());
    }

}
