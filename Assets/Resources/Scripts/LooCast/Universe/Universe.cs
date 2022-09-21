using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;

namespace LooCast.Universe
{
    using LooCast.Filament;
    using LooCast.Sector;
    using LooCast.Region;

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
        //How many Filaments fit into the Universe (Per Axis)
        [SerializeField] private int universeSize;

        //A list of all voids. These define the most basic structure of the universe and are all generated at once
        private Vector2Int[] voids;
        #endregion

        #region Filaments
        //How many Sectors fit into a Filament (Per Axis)
        [SerializeField] private int filamentSize;

        [SerializeField] private GameObject filamentPrefab;
        #endregion

        #region Sectors
        //How many Regions fit into a Sector (Per Axis)
        [SerializeField] private int sectorSize;

        [SerializeField] private GameObject sectorPrefab;
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

        private Dictionary<Vector2Int, Filament> loadedFilaments = new Dictionary<Vector2Int, Filament>();
        private Dictionary<Vector2Int, Sector> loadedSectors = new Dictionary<Vector2Int, Sector>();
        private Dictionary<Vector2Int, Region> loadedRegions = new Dictionary<Vector2Int, Region>();

        private void Start()
        {

        }

        #region Filaments

        #region Utility
        public Filament GetFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is not loaded!");
            }

            if (!IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is not generated!");
            }

            return loadedFilaments[filamentPosition];
        }
        #endregion

        #region Generation
        private bool IsFilamentGenerated(Vector2Int filamentPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            return File.Exists(path);
        }

        private void GenerateFilament(Vector2Int filamentPosition)
        {
            if (IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is already generated!");
            }
            Filament filament = new Filament(filamentPosition, filamentSize);
            SaveFilament(filament);
        }
        #endregion

        #region Saving
        private void SaveFilament(Filament filament)
        {
            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filament.FilamentPosition.x}.{filament.FilamentPosition.y}.json";
            string json = JsonUtility.ToJson(filament, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        private bool IsFilamentLoaded(Vector2Int filamentPosition)
        {
            return loadedFilaments.ContainsKey(filamentPosition);
        }

        private void LoadFilament(Vector2Int filamentPosition)
        {
            if (IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already loaded!");
            }

            if (!IsFilamentGenerated(filamentPosition))
            {
                throw new Exception($"Filament has not been generated yet!");
            }

            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            loadedFilaments.Add(filamentPosition, JsonUtility.FromJson<Filament>(json));
        }

        private void UnloadFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already unloaded!");
            }

            loadedFilaments.Remove(filamentPosition);
        }
        #endregion

        #region Spawning
        private void SpawnFilament(Vector2Int filamentPosition)
        {
            GetFilament(filamentPosition).Spawn(filamentPrefab);
        }

        private void DespawnFilament(Vector2Int filamentPosition)
        {
            GetFilament(filamentPosition).Despawn();
        }
        #endregion

        #endregion

        #region Sectors

        #region Utility
        public Sector GetSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is not loaded!");
            }

            if (!IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Sector is not generated!");
            }

            return loadedSectors[sectorPosition];
        }
        #endregion

        #region Generation
        private bool IsSectorGenerated(Vector2Int sectorPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            return File.Exists(path);
        }

        private void GenerateSector(Vector2Int sectorPosition)
        {
            if (IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Sector is already generated!");
            }
            Sector sector = new Sector(sectorPosition, sectorSize);
            SaveSector(sector);
        }
        #endregion

        #region Saving
        private void SaveSector(Sector sector)
        {
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sector.SectorPosition.x}.{sector.SectorPosition.y}.json";
            string json = JsonUtility.ToJson(sector, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        private bool IsSectorLoaded(Vector2Int sectorPosition)
        {
            return loadedSectors.ContainsKey(sectorPosition);
        }

        private void LoadSector(Vector2Int sectorPosition)
        {
            if (IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already loaded!");
            }

            if (!IsSectorGenerated(sectorPosition))
            {
                throw new Exception($"Sector has not been generated yet!");
            }

            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            loadedSectors.Add(sectorPosition, JsonUtility.FromJson<Sector>(json));
        }

        private void UnloadSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already unloaded!");
            }

            loadedSectors.Remove(sectorPosition);
        }
        #endregion

        #region Spawning
        private void SpawnSector(Vector2Int sectorPosition)
        {
            GetSector(sectorPosition).Spawn(sectorPrefab);
        }

        private void DespawnSector(Vector2Int sectorPosition)
        {
            GetSector(sectorPosition).Despawn();
        }
        #endregion

        #endregion

        #region Regions

        #region Utility
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
        #endregion

        #region Generation
        private bool IsRegionGenerated(Vector2Int regionPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Regions/{regionPosition.x}.{regionPosition.y}.json";
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

        #region Saving
        private void SaveRegion(Region region)
        {
            string path = $"{Application.dataPath}/Data/Universe/Regions/{region.RegionPosition.x}.{region.RegionPosition.y}.json";
            string json = JsonUtility.ToJson(region, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
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

            string path = $"{Application.dataPath}/Data/Universe/Regions/{regionPosition.x}.{regionPosition.y}.json";
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

        #region Spawning
        private void SpawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Spawn(regionPrefab);
        }

        private void DespawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Despawn();
        }
        #endregion

        #endregion
    }
}