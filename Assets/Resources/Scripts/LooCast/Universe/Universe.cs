using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;
using System.Linq;

namespace LooCast.Universe
{
    using Void;
    using Filament;
    using Sector;
    using Region;
    using LooCast.Random;

    public class Universe
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            public int seed;
            public float scale;
            public float amplitude;
            public int octaves;
            public float persistence;
            public float lacunarity;
        }
        #endregion

        #region Universe
        public static Universe Instance => instance;
        private static Universe instance;

        [SerializeField] private GenerationSettings generationSettings;
        //How many Filaments fit into the Universe (Per Axis)
        [SerializeField] private int universeSize;

        #region Voids
        //How many voids fit into the Universe (Per Axis)
        [SerializeField] private int voidAmount;
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
        #endregion
        #endregion

        private Dictionary<Vector2Int, Void.Void> loadedVoids = new Dictionary<Vector2Int, Void.Void>();
        private Dictionary<Vector2Int, Filament.Filament> loadedFilaments = new Dictionary<Vector2Int, Filament.Filament>();
        private Dictionary<Vector2Int, Sector.Sector> loadedSectors = new Dictionary<Vector2Int, Sector.Sector>();
        private Dictionary<Vector2Int, Region.Region> loadedRegions = new Dictionary<Vector2Int, Region.Region>();

        private Universe
        (
            GenerationSettings generationSettings, 
            int universeSize, 
            int voidAmount, 
            int filamentSize, 
            int sectorSize, 
            int regionSize, 
            GameObject filamentPrefab, 
            GameObject sectorPrefab,
            GameObject regionPrefab
        )
        {
            this.generationSettings = generationSettings; 
            this.universeSize = universeSize; 
            this.voidAmount = voidAmount; 
            this.filamentSize = filamentSize; 
            this.sectorSize = sectorSize; 
            this.regionSize = regionSize; 
            this.filamentPrefab = filamentPrefab; 
            this.sectorPrefab = sectorPrefab;
            this.regionPrefab = regionPrefab;
        }

        #region Universe

        #region Utility
        public static void Generate
        (
            GenerationSettings generationSettings, 
            int universeSize, 
            int voidAmount, 
            int filamentSize, 
            int sectorSize, 
            int regionSize, 
            GameObject filamentPrefab, 
            GameObject sectorPrefab,
            GameObject regionPrefab
        )
        {
            if (IsUniverseGenerated())
            {
                throw new Exception($"Universe has already been generated!");
            }

            if (IsUniverseLoaded())
            {
                throw new Exception("Universe is already loaded!");
            }

            GenerateUniverse
            (
                generationSettings, 
                universeSize, 
                voidAmount, 
                filamentSize, 
                sectorSize, 
                regionSize, 
                filamentPrefab, 
                sectorPrefab,
                regionPrefab
            );
            SaveUniverse();
        }

        public static void Save()
        {
            if (!IsUniverseLoaded())
            {
                throw new Exception("Universe is not loaded!");
            } 

            SaveUniverse();
        }

        public static void Load()
        {
            if (IsUniverseLoaded())
            {
                throw new Exception("Universe is already loaded!");
            }

            if (!IsUniverseGenerated())
            {
                throw new Exception($"Universe has not been generated yet!");
            }

            LoadUniverse();
        }

        public static void Unload()
        {
            if (!IsUniverseLoaded())
            {
                throw new Exception("Universe is already unloaded!");
            }

            UnloadUniverse();
        }
        #endregion

        #region Generation
        private static bool IsUniverseGenerated()
        {
            string path = $"{Application.dataPath}/Data/Universe/Universe.json";
            return File.Exists(path);
        }
        
        private static void GenerateUniverse
        (
            GenerationSettings generationSettings, 
            int universeSize, 
            int voidAmount, 
            int filamentSize, 
            int sectorSize, 
            int regionSize, 
            GameObject filamentPrefab, 
            GameObject sectorPrefab,
            GameObject regionPrefab
        )
        {
            Universe universe = new Universe
            (
                generationSettings, 
                universeSize, 
                voidAmount, 
                filamentSize, 
                sectorSize, 
                regionSize, 
                filamentPrefab, 
                sectorPrefab,
                regionPrefab
            );

            SeededRandom prng = new SeededRandom(universe.generationSettings.seed);
            for (int x = 0; x < universe.voidAmount; x++)
            {
                for (int y = 0; y < universe.voidAmount; y++)
                {
                    Vector2Int voidPosition = new Vector2Int(x, y);
                    Vector2 normalizedVoidPositionOffset = new Vector2(prng.Range(-0.5f, 0.5f), prng.Range(-0.5f, 0.5f));
                    universe.GenerateVoid(voidPosition, normalizedVoidPositionOffset);
                    universe.LoadVoid(voidPosition);
                }
            }

            instance = universe;
        }
        #endregion

        #region Saving
        private static void SaveUniverse()
        {
            string path = $"{Application.dataPath}/Data/Universe/Universe.json";
            string json = JsonUtility.ToJson(Instance, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        private static bool IsUniverseLoaded()
        {
            return instance != null;
        }

        private static void LoadUniverse()
        {
            string path = $"{Application.dataPath}/Data/Universe/Universe.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            instance = JsonUtility.FromJson<Universe>(json);
        }

        private static void UnloadUniverse()
        {
            instance = null;
        }
        #endregion

        #endregion

        #region Voids

        #region Utility
        public Void.Void[] GetVoids()
        {
            return loadedVoids.Values.ToArray();
        }
        
        public Void.Void GetVoid(Vector2Int voidPosition)
        {
            if (!IsVoidLoaded(voidPosition))
            {
                throw new Exception("Void is not loaded!");
            }

            if (!IsVoidGenerated(voidPosition))
            {
                throw new Exception("Void is not generated!");
            }

            return loadedVoids[voidPosition];
        }
        #endregion

        #region Generation
        private bool IsVoidGenerated(Vector2Int voidPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Voids/{voidPosition.x}.{voidPosition.y}.json";
            return File.Exists(path);
        }

        private void GenerateVoid(Vector2Int voidPosition, Vector2 normalizedVoidPositionOffset)
        {
            if (IsVoidGenerated(voidPosition))
            {
                throw new Exception("Void is already generated!");
            }
            Void.Void @void = new Void.Void(voidPosition, normalizedVoidPositionOffset);
            SaveVoid(@void);
        }
        #endregion
        
        #region Saving
        private void SaveVoid(Void.Void @void)
        {
            string path = $"{Application.dataPath}/Data/Universe/Voids/{@void.VoidPosition.x}.{@void.VoidPosition.y}.json";
            string json = JsonUtility.ToJson(@void, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        private bool IsVoidLoaded(Vector2Int voidPosition)
        {
            return loadedVoids.ContainsKey(voidPosition);
        }

        private void LoadVoid(Vector2Int voidPosition)
        {
            if (IsVoidLoaded(voidPosition))
            {
                throw new Exception("Void is already loaded!");
            }

            if (!IsVoidGenerated(voidPosition))
            {
                throw new Exception($"Void has not been generated yet!");
            }

            string path = $"{Application.dataPath}/Data/Universe/Voids/{voidPosition.x}.{voidPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            loadedVoids.Add(voidPosition, JsonUtility.FromJson<Void.Void>(json));
        }

        private void UnloadVoid(Vector2Int voidPosition)
        {
            if (!IsVoidLoaded(voidPosition))
            {
                throw new Exception("Void is already unloaded!");
            }

            loadedVoids.Remove(voidPosition);
        }
        #endregion

        #endregion

        #region Filaments

        #region Utility
        public Filament.Filament GetFilament(Vector2Int filamentPosition)
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
            Filament.Filament filament = new Filament.Filament(filamentPosition, filamentSize, generationSettings);
            SaveFilament(filament);
        }
        #endregion

        #region Saving
        private void SaveFilament(Filament.Filament filament)
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
            loadedFilaments.Add(filamentPosition, JsonUtility.FromJson<Filament.Filament>(json));
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
        public Sector.Sector GetSector(Vector2Int sectorPosition)
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
            Sector.Sector sector = new Sector.Sector(sectorPosition, sectorSize, generationSettings);
            SaveSector(sector);
        }
        #endregion

        #region Saving
        private void SaveSector(Sector.Sector sector)
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
            loadedSectors.Add(sectorPosition, JsonUtility.FromJson<Sector.Sector>(json));
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
        public Region.Region GetRegion(Vector2Int regionPosition)
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
            Region.Region region = new Region.Region(regionPosition, regionSize, generationSettings);
            SaveRegion(region);
        }
        #endregion

        #region Saving
        private void SaveRegion(Region.Region region)
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
            loadedRegions.Add(regionPosition, JsonUtility.FromJson<Region.Region>(json));
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