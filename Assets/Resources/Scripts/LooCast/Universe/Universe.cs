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
            //How many Filaments fit into the Universe (Per Axis)
            public int size;
            
            public Void.Void.GenerationSettings voidGenerationSettings;
            public Filament.Filament.GenerationSettings filamentGenerationSettings;
            public Sector.Sector.GenerationSettings sectorGenerationSettings;
            public Region.Region.GenerationSettings regionGenerationSettings;
        }
        #endregion

        public static Universe Instance => instance;
        private static Universe instance;

        [SerializeField] GenerationSettings generationSettings;

        private Dictionary<Vector2Int, Void.Void> loadedVoids = new Dictionary<Vector2Int, Void.Void>();
        private Dictionary<Vector2Int, Filament.Filament> loadedFilaments = new Dictionary<Vector2Int, Filament.Filament>();
        private Dictionary<Vector2Int, Sector.Sector> loadedSectors = new Dictionary<Vector2Int, Sector.Sector>();
        private Dictionary<Vector2Int, Region.Region> loadedRegions = new Dictionary<Vector2Int, Region.Region>();

        private Universe(GenerationSettings generationSettings)
        {
            this.generationSettings = generationSettings;
        }

        #region Universe

        #region Generation
        public static bool IsUniverseGenerated()
        {
            string path = $"{Application.dataPath}/Data/Universe/Universe.json";
            return File.Exists(path);
        }
        
        public static void GenerateUniverse(GenerationSettings generationSettings)
        {
            if (IsUniverseGenerated())
            {
                throw new Exception($"Universe has already been generated!");
            }

            if (IsUniverseLoaded())
            {
                throw new Exception("Universe is already loaded!");
            }

            Universe universe = new Universe(generationSettings);

            SeededRandom prng = new SeededRandom(universe.generationSettings.seed);
            for (int x = 0; x < universe.generationSettings.voidGenerationSettings.amount; x++)
            {
                for (int y = 0; y < universe.generationSettings.voidGenerationSettings.amount; y++)
                {
                    Vector2Int voidPosition = new Vector2Int(x, y);
                    Vector2 normalizedVoidPositionOffset = new Vector2(prng.Range(-0.5f, 0.5f), prng.Range(-0.5f, 0.5f));
                    universe.GenerateVoid(voidPosition, normalizedVoidPositionOffset);
                    universe.LoadVoid(voidPosition);
                }
            }

            instance = universe;
            SaveUniverse();
        }
        #endregion

        #region Saving
        public static void SaveUniverse()
        {
            if (!IsUniverseLoaded())
            {
                throw new Exception("Universe is not loaded!");
            }

            string path = $"{Application.dataPath}/Data/Universe/Universe.json";
            string json = JsonUtility.ToJson(Instance, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);

            
        }
        #endregion

        #region Loading
        public static bool IsUniverseLoaded()
        {
            return instance != null;
        }

        public static void LoadUniverse()
        {
            if (IsUniverseLoaded())
            {
                throw new Exception("Universe is already loaded!");
            }

            if (!IsUniverseGenerated())
            {
                throw new Exception($"Universe has not been generated yet!");
            }

            string path = $"{Application.dataPath}/Data/Universe/Universe.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            instance = JsonUtility.FromJson<Universe>(json);
        }

        public static void UnloadUniverse()
        {
            if (!IsUniverseLoaded())
            {
                throw new Exception("Universe is already unloaded!");
            }

            instance = null;
        }
        #endregion

        #region Deletion
        public static void DeleteUniverse()
        {
            if (IsUniverseLoaded())
            {
                UnloadUniverse();
            }

            string path = $"{Application.dataPath}/Data/Universe";
            if (Directory.Exists(path))
            {
                DirectoryInfo directoryInfo = new DirectoryInfo(path);
                directoryInfo.Delete(true);
            }
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

            Void.Void @void = new Void.Void(generationSettings, voidPosition, normalizedVoidPositionOffset);
            SaveVoid(@void);
        }
        #endregion
        
        #region Saving
        private void SaveVoid(Vector2Int voidPosition)
        {
            if (!IsVoidLoaded(voidPosition))
            {
                throw new Exception("Void is not loaded!");
            }

            Void.Void @void = GetVoid(voidPosition);
            string path = $"{Application.dataPath}/Data/Universe/Voids/{voidPosition.x}.{voidPosition.y}.json";
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

        #region Deletion
        private void DeleteVoids()
        {
            foreach (Void.Void @void in loadedVoids.Values)
            {
                UnloadVoid(@void.VoidPosition);
            }

            string path = $"{Application.dataPath}/Data/Universe/Voids";
            if (Directory.Exists(path))
            {
                DirectoryInfo directoryInfo = new DirectoryInfo(path);
                directoryInfo.Delete(true);
            }
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
        public bool IsFilamentGenerated(Vector2Int filamentPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateFilament(Vector2Int filamentPosition)
        {
            if (IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is already generated!");
            }

            Filament.Filament filament = new Filament.Filament(generationSettings, filamentPosition);
            SaveFilament(filament);
        }
        #endregion

        #region Saving
        public void SaveFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is not loaded!");
            }

            Filament.Filament filament = GetFilament(filamentPosition);
            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            string json = JsonUtility.ToJson(filament, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        public bool IsFilamentLoaded(Vector2Int filamentPosition)
        {
            return loadedFilaments.ContainsKey(filamentPosition);
        }

        public void LoadFilament(Vector2Int filamentPosition)
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

        public void UnloadFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already unloaded!");
            }

            loadedFilaments.Remove(filamentPosition);
        }
        #endregion

        #region Spawning
        public void SpawnFilament(Vector2Int filamentPosition)
        {
            GetFilament(filamentPosition).Spawn();
        }

        public void DespawnFilament(Vector2Int filamentPosition)
        {
            GetFilament(filamentPosition).Despawn();
        }
        #endregion

        #region Deletion
        public void DeleteFilament(Vector2Int filamentPosition)
        {
            if (IsFilamentLoaded(filamentPosition))
            {
                UnloadFilament(filamentPosition);
            }

            if (IsFilamentGenerated(filamentPosition))
            {
                string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
                File.Delete(path);
            }
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
        public bool IsSectorGenerated(Vector2Int sectorPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateSector(Vector2Int filamentPosition, Vector2Int sectorPosition)
        {
            if (!IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is not generated yet!");
            }
            if (IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Sector is already generated!");
            }

            Sector.Sector sector = new Sector.Sector(generationSettings, filamentPosition, sectorPosition);
            SaveSector(sector);
        }
        #endregion

        #region Saving
        public void SaveSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is not loaded!");
            }

            Sector.Sector sector = GetSector(sectorPosition);
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            string json = JsonUtility.ToJson(sector, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        public bool IsSectorLoaded(Vector2Int sectorPosition)
        {
            return loadedSectors.ContainsKey(sectorPosition);
        }

        public void LoadSector(Vector2Int sectorPosition)
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

        public void UnloadSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already unloaded!");
            }

            loadedSectors.Remove(sectorPosition);
        }
        #endregion

        #region Spawning
        public void SpawnSector(Vector2Int sectorPosition)
        {
            GetSector(sectorPosition).Spawn();
        }

        public void DespawnSector(Vector2Int sectorPosition)
        {
            GetSector(sectorPosition).Despawn();
        }
        #endregion

        #region Deletion
        public void DeleteSector(Vector2Int sectorPosition)
        {
            if (IsSectorLoaded(sectorPosition))
            {
                UnloadSector(sectorPosition);
            }

            if (IsSectorGenerated(sectorPosition))
            {
                string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
                File.Delete(path);
            }
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
        public bool IsRegionGenerated(Vector2Int regionPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Regions/{regionPosition.x}.{regionPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateRegion(Vector2Int sectorPosition, Vector2Int regionPosition)
        {
            if (!IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Sector is not generated yet!");
            }
            if (IsRegionGenerated(regionPosition))
            {
                throw new Exception("Region is already generated!");
            }

            Region.Region region = new Region.Region(generationSettings, sectorPosition, regionPosition);
            SaveRegion(region);
        }
        #endregion

        #region Saving
        public void SaveRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is not loaded!");
            }

            Region.Region region = GetRegion(regionPosition);
            string path = $"{Application.dataPath}/Data/Universe/Regions/{regionPosition.x}.{regionPosition.y}.json";
            string json = JsonUtility.ToJson(region, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }
        #endregion

        #region Loading
        public bool IsRegionLoaded(Vector2Int regionPosition)
        {
            return loadedRegions.ContainsKey(regionPosition);
        }

        public void LoadRegion(Vector2Int regionPosition)
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

        public void UnloadRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already unloaded!");
            }

            loadedRegions.Remove(regionPosition);
        }
        #endregion

        #region Spawning
        public void SpawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Spawn();
        }

        public void DespawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Despawn();
        }
        #endregion

        #region Deletion
        public void DeleteRegion(Vector2Int regionPosition)
        {
            if (IsRegionLoaded(regionPosition))
            {
                UnloadRegion(regionPosition);
            }

            if (IsRegionGenerated(regionPosition))
            {
                string path = $"{Application.dataPath}/Data/Universe/Regions/{regionPosition.x}.{regionPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion
    }
}