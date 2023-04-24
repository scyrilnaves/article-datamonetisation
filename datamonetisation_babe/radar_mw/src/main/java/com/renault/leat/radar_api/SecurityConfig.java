package com.renault.leat.radar_api;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.annotation.Order;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.WebSecurityConfigurerAdapter;
import org.springframework.security.config.annotation.web.configurers.oauth2.server.resource.OAuth2ResourceServerConfigurer;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.provisioning.InMemoryUserDetailsManager;
import org.springframework.stereotype.Component;

@Component
public class SecurityConfig extends WebSecurityConfigurerAdapter {

    private final PasswordEncoder passwordEncoder;

    public SecurityConfig(PasswordEncoder passwordEncoder) {
        this.passwordEncoder = passwordEncoder;
    }

    @Override
    protected void configure(HttpSecurity http) throws Exception {
        http.cors().and().csrf().disable().sessionManagement().sessionCreationPolicy(SessionCreationPolicy.STATELESS)
                .and()
                .authorizeRequests(configurer -> configurer
                        .antMatchers("/error", "/login", "/assettoken", "/assetservicetoken", "/postassettoken",
                                "/postassetservicetoken", "/postData", "/getData", "/assetexists",
                                "/assetserviceexists",
                                "/getassettokens", "/getassetservicetokens", "/getdeleteassetid",
                                "/getdeleteassetserviceid", "/getPublicKey", "/getEncryptedBid", "/getDecryptedBid",
                                "/isDeployed", "/deployementStatus", "/actuator/mappings")
                        .permitAll().anyRequest().authenticated())
                .exceptionHandling().disable().oauth2ResourceServer(OAuth2ResourceServerConfigurer::jwt);
    }

    // Spring has UserDetails Service used to retrieve user data and check against
    // username and password
    @Bean
    @Override
    protected UserDetailsService userDetailsService() {
        UserDetails user1 = User.withUsername("radar_oem").authorities("USER").passwordEncoder(passwordEncoder::encode)
                .password("leat_radar").build();
        InMemoryUserDetailsManager manager = new InMemoryUserDetailsManager();
        manager.createUser(user1);
        return manager;
    }

}
