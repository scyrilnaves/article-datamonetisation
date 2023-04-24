package com.renault.leat.radar_api;

public class RadarData {

    private String carId;
    private String radarDetails;

    public void setCarId(String carId) {
        this.carId = carId;
    }

    public String getCarId() {
        return this.carId;
    }

    public void setRadarDetails(String radarDetail) {
        this.radarDetails = radarDetail;
    }

    public String getRadarDetails() {
        return this.radarDetails;
    }

    @Override
    public String toString() {
        return "Radar Data [car detail = " + carId + ", email=" + radarDetails + "]";
    }

}
