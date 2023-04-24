package com.renault.leat.radar_api;

import java.util.Map;

import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationToken;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/user")
public class UserController {

    private final UserDetailsService userDetailsService;

    public UserController(UserDetailsService userDetailsService) {
        this.userDetailsService = userDetailsService;
    }

    @GetMapping
    public UserDetails getUser(Authentication authentication) {
        JwtAuthenticationToken token = (JwtAuthenticationToken) authentication;
        Map<String, Object> attributes = token.getTokenAttributes();
        return userDetailsService.loadUserByUsername(attributes.get("username").toString());
    }

    /**
     * Method to get the data by utilising the access Token
     * 
     * @return
     */
    @RequestMapping("/api/radardata")
    public ResponseEntity<RadarData> getRadarData() {
        RadarData radarData = new RadarData();
        radarData.setCarId("FR4567");
        radarData.setRadarDetails("LAT: 22E34N , LON: 34N");
        return ResponseEntity.ok(radarData);
    }

    /**
     * Method to get the data by utilising the access Token
     * 
     * @return
     */
    @RequestMapping("/api/radarservice")
    public ResponseEntity<RadarData> getRadarService() {
        RadarData radarData = new RadarData();
        radarData.setCarId("FR4567");
        radarData.setRadarDetails("LAT: 22E34N , LON: 34N");
        return ResponseEntity.ok(radarData);
    }

}
