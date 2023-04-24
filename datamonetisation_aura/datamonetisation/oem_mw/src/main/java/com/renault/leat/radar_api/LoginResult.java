package com.renault.leat.radar_api;

import lombok.Data;
import lombok.NonNull;
import lombok.RequiredArgsConstructor;

@Data
public class LoginResult {

    @NonNull
    private String jwt;

    public LoginResult(String jwt) {
        this.jwt = jwt;
    }

    public String getJWT() {
        return this.jwt;
    }

}
