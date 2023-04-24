package com.renault.leat.radar_api;

public class AssetFeedback {

    private String assetId;
    private String assetToken;

    public void setAssetId(String assetId) {
        this.assetId = assetId;
    }

    public String getAssetId() {
        return this.assetId;
    }

    public void setAssetToken(String assetToken) {
        this.assetToken = assetToken;
    }

    public String getAssetToken() {
        return this.assetToken;
    }

    @Override
    public String toString() {
        return "Asset Data [assetId = " + assetId + ", assetToken=" + assetToken + "]";
    }

}
