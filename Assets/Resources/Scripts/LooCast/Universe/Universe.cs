using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;

namespace LooCast.Universe
{
    public class Universe : MonoBehaviour
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            //Perlin
            public int seed;
            public float scale;
            public float amplitude;
            public int octaves;
            public float persistence;
            public float lacunarity;
        }
        #endregion

        #region Universe
        //How many Filament Chunks fit into the Universe (Per Axis)
        [SerializeField] private int universeSize;

        //A list of all voids. These define the most basic structure of the universe and are all generated at once
        private Vector2Int[] voids;
        #endregion

        #region Filaments
        //How many Chunks fit into a Filament Chunk (Per Axis)
        [SerializeField] private int filamentSize;
        #endregion

        #region Sectors
        //How many Regions fit into a Sector (Per Axis)
        [SerializeField] private int sectorSize;
        #endregion

        #region Regions
        //How many Units fit into a Region (Per Axis)
        [SerializeField] private int regionSize;

        [SerializeField] private GameObject regionPrefab;
        [SerializeField] private GenerationSettings generationSettings;
        #endregion

        #region DEVELOPMENT
        [SerializeField] private Vector2Int[] chunkCoordinates;
        #endregion

        private Dictionary<Vector2Int, Region> loadedRegions = new Dictionary<Vector2Int, Region>();

        private void Start()
        {

        }

        public void DEV_GenerateChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                GenerateRegion(chunkCoordinate);
            }
        }

        public void DEV_LoadChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                LoadRegion(chunkCoordinate);
            }
        }

        public void DEV_UnloadChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                UnloadRegion(chunkCoordinate);
            }
        }

        public void DEV_SpawnChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                SpawnRegion(chunkCoordinate);
            }
        }

        public void DEV_DespawnChunks()
        {
            foreach (Vector2Int chunkCoordinate in chunkCoordinates)
            {
                DespawnRegion(chunkCoordinate);
            }
        }

        #region Filament Chunks

        #endregion

        #region Regions
        public Region GetRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is not loaded!");
            }

            if (!IsRegionGenerated(regionPosition))
            {
                throw new Exception("Region is not generated!");
            }

            return loadedRegions[regionPosition];
        }

        #region Region Generation
        private bool IsRegionGenerated(Vector2Int regionPosition)
        {
            string path = $"{Application.dataPath}/Data/World/Regions/{regionPosition.x}.{regionPosition.y}.json";
            return File.Exists(path);
        }

        private void GenerateRegion(Vector2Int regionPosition)
        {
            if (IsRegionGenerated(regionPosition))
            {
                throw new Exception("Region is already generated!");
            }
            Region region = new Region(regionPosition, regionSize, generationSettings);
            SaveRegion(region);
        }
        #endregion

        #region Region Saving
        private void SaveRegion(Region region)
        {
            string path = $"{Application.dataPath}/Data/World/Regions/{region.RegionPosition.x}.{region.RegionPosition.y}.json";
            string json = JsonUtility.ToJson(region, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Region Loading
        private bool IsRegionLoaded(Vector2Int regionPosition)
        {
            return loadedRegions.ContainsKey(regionPosition);
        }

        private void LoadRegion(Vector2Int regionPosition)
        {
            if (IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already loaded!");
            }

            if (!IsRegionGenerated(regionPosition))
            {
                throw new Exception($"Region has not been generated yet!");
            }

            string path = $"{Application.dataPath}/Data/World/Regions/{regionPosition.x}.{regionPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            loadedRegions.Add(regionPosition, JsonUtility.FromJson<Region>(json));
        }

        private void UnloadRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already unloaded!");
            }

            loadedRegions.Remove(regionPosition);
        }
        #endregion

        #region Region Spawning
        private void SpawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Spawn(regionPrefab);
        }
        #endregion

        #region Region Despawning
        private void DespawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Despawn();
        }
        #endregion
        #endregion
    }
}