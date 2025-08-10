package br.com.mili.backend.security;

import br.com.mili.backend.model.User;
import br.com.mili.backend.repository.UserRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class UserDetailsServiceImpl implements UserDetailsService {

    private static final Logger logger = LoggerFactory.getLogger(UserDetailsServiceImpl.class);
    private final UserRepository repository;

    public UserDetailsServiceImpl(UserRepository repository) {
        this.repository = repository;
    }

    @Override
    public UserDetails loadUserByUsername(String email) throws UsernameNotFoundException {
        logger.info("Tentando autenticar o usuário com email: {}", email);
        User user = repository.findByEmail(email).orElseThrow(() -> {
            logger.error("Usuário com email '{}' não encontrado no banco de dados.", email);
            return new UsernameNotFoundException("Usuário não encontrado!");
        });
        logger.info("Usuário encontrado: {}. Hash da senha do banco: {}", user.getEmail(), user.getPassword());

        return new org.springframework.security.core.userdetails.User(
                user.getEmail(),
                user.getPassword(),
                List.of(new SimpleGrantedAuthority("ROLE_USER"))
        );
    }
}
