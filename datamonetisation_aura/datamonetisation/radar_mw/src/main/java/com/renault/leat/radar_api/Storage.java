package com.renault.leat.radar_api;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

import com.renault.leat.radar_api.AssetFeedback;

public class Storage {

    // Storage to store the Asset Id and Asset Service Id
    static List<Integer> assetIds = new ArrayList<Integer>();

    static List<Integer> assetServiceIds = new ArrayList<Integer>();

    static List<AssetFeedback> assetTokens = new ArrayList<AssetFeedback>();

    static List<AssetFeedback> assetServiceTokens = new ArrayList<AssetFeedback>();

    static List<RadarData> assetRadarList = new ArrayList<RadarData>();

    static AtomicInteger assetIndex = new AtomicInteger(1);

    static AtomicInteger assetServiceIndex = new AtomicInteger(1);

    public static void incrementAssetIndex() {
        assetIndex.incrementAndGet();
    }

    public static int getAssetIndex() {
        return assetIndex.get();
    }

    public static void removeAssetId(List<Integer> astId) {
        assetIds.removeAll(astId);
    }

    public static void removeAssetServiceId(List<Integer> astServiceId) {
        assetServiceIds.removeAll(astServiceId);
    }

    public static void incrementAssetServiceIndex() {
        assetServiceIndex.incrementAndGet();
    }

    public static int getAssetServiceIndex() {
        return assetServiceIndex.get();
    }

    public static List<Integer> getAssetIds() {
        return assetIds;
    }

    public static void setAssetTokens(List<AssetFeedback> inAssetTokens) {
        assetTokens = inAssetTokens;
    }

    public static void addAssetToken(AssetFeedback inAssetFeedback) {
        assetTokens.add(inAssetFeedback);
    }

    public static List<AssetFeedback> getAssetTokens() {
        return assetTokens;
    }

    public static void setAssetServiceTokens(List<AssetFeedback> inAssetServiceTokens) {
        assetServiceTokens = inAssetServiceTokens;
    }

    public static void addAssetServiceToken(AssetFeedback inAssetFeedback) {
        assetServiceTokens.add(inAssetFeedback);
    }

    public static List<AssetFeedback> getAssetServiceTokens() {
        return assetServiceTokens;
    }

    public static void setAssetIds(List<Integer> inassetIds) {
        assetIds = inassetIds;
    }

    public static boolean containsAssetId(String assetId) {
        return assetIds.contains(assetId);
    }

    public static boolean containsAssetServiceId(String assetServiceId) {
        return assetServiceIds.contains(assetServiceId);
    }

    public static List<Integer> getAssetServiceIds() {
        return assetServiceIds;
    }

    public static void setAssetServiceIds(List<Integer> inassetServiceIds) {
        assetServiceIds = inassetServiceIds;
    }

    public static void addAssetId(Integer inAssetId) {
        assetIds.add(inAssetId);

    }

    public static void addAssetServiceId(Integer inAssetServiceId) {
        assetServiceIds.add(inAssetServiceId);
    }

    public static void addRadarData(RadarData radarData) {
        assetRadarList.add(radarData);
    }

    public static RadarData getRadarData() {
        return assetRadarList.get(0);
    }

}
