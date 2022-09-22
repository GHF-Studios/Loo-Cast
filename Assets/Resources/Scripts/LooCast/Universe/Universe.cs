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

        public GenerationSettings UniverseGenerationSettings => generationSettings;
        public Void.Void.GenerationSettings VoidGenerationSettings => generationSettings.voidGenerationSettings;
        public Filament.Filament.GenerationSettings FilamentGenerationSettings => generationSettings.filamentGenerationSettings;
        public Sector.Sector.GenerationSettings SectorGenerationSettings => generationSettings.sectorGenerationSettings;
        public Region.Region.GenerationSettings RegionGenerationSettings => generationSettings.regionGenerationSettings;

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

            Instance.SaveFilaments();
            Instance.SaveSectors();
            Instance.SaveRegions();
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
        
        public Void.Void[] GetVoids(Vector2Int[] voidPositions)
        {
            Void.Void[] voids = new Void.Void[voidPositions.Length];
            for (int i = 0; i < voidPositions.Length; i++)
            {
                voids[i] = GetVoid(voidPositions[i]);
            }
            return voids;
        }

        public Void.Void[] GetVoids()
        {
            return loadedVoids.Values.ToArray();
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

            loadedVoids.Add(voidPosition, new Void.Void(voidPosition, normalizedVoidPositionOffset));
            SaveVoid(voidPosition);
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

        public Filament.Filament[] GetFilaments(Vector2Int[] filamentPositions)
        {
            Filament.Filament[] filaments = new Filament.Filament[filamentPositions.Length];
            for (int i = 0; i < filamentPositions.Length; i++)
            {
                filaments[i] = GetFilament(filamentPositions[i]);
            }
            return filaments;
        }

        public Filament.Filament[] GetFilaments()
        {
            return loadedFilaments.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsFilamentGenerated(Vector2Int filamentPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            return File.Exists(path);
        }

        public bool AreFilamentsGenerated(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                if (!IsFilamentGenerated(filamentPosition))
                {
                    return false;
                }
            }
            return true;
        }

        public void GenerateFilament(Vector2Int filamentPosition)
        {
            if (IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is already generated!");
            }

            loadedFilaments.Add(filamentPosition, new Filament.Filament(filamentPosition));
            SaveFilament(filamentPosition);
        }
        
        public void GenerateFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                GenerateFilament(filamentPosition);
            }
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
        
        public void SaveFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                SaveFilament(filamentPosition);
            }
        }

        public void SaveFilaments()
        {
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys)
            {
                SaveFilament(filamentPosition);
            }
        }
        #endregion

        #region Loading
        public bool IsFilamentLoaded(Vector2Int filamentPosition)
        {
            return loadedFilaments.ContainsKey(filamentPosition);
        }

        public bool AreFilamentsLoaded(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                if (!IsFilamentLoaded(filamentPosition))
                {
                    return false;
                }
            }
            return true;
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

        public void LoadFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                LoadFilament(filamentPosition);
            }
        }

        public void UnloadFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already unloaded!");
            }

            loadedFilaments.Remove(filamentPosition);
        }
        
        public void UnloadFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                UnloadFilament(filamentPosition);
            }
        }
        
        public void UnloadFilaments()
        {
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys)
            {
                UnloadFilament(filamentPosition);
            }
        }
        #endregion

        #region Spawning
        public void SpawnFilament(Vector2Int filamentPosition)
        {
            GetFilament(filamentPosition).Spawn();
        }

        public void SpawnFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                SpawnFilament(filamentPosition);
            }
        }

        public void DespawnFilament(Vector2Int filamentPosition)
        {
            GetFilament(filamentPosition).Despawn();
        }
        
        public void DespawnFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                DespawnFilament(filamentPosition);
            }
        }
        
        public void DespawnFilaments()
        {
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys)
            {
                DespawnFilament(filamentPosition);
            }
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

        public void DeleteFilaments(Vector2Int[] filamentPositions)
        {
            foreach (Vector2Int filamentPosition in filamentPositions)
            {
                DeleteFilament(filamentPosition);
            }
        }
        
        public void DeleteFilaments()
        {
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys)
            {
                DeleteFilament(filamentPosition);
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

        public Sector.Sector[] GetSectors(Vector2Int[] sectorPositions)
        {
            Sector.Sector[] sectors = new Sector.Sector[sectorPositions.Length];
            for (int i = 0; i < sectorPositions.Length; i++)
            {
                sectors[i] = GetSector(sectorPositions[i]);
            }
            return sectors;
        }

        public Sector.Sector[] GetSectors()
        {
            return loadedSectors.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsSectorGenerated(Vector2Int sectorPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            return File.Exists(path);
        }

        public bool AreSectorsGenerated(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                if (!IsSectorGenerated(sectorPosition))
                {
                    return false;
                }
            }
            return true;
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

            loadedSectors.Add(sectorPosition, new Sector.Sector(filamentPosition, sectorPosition));
            SaveSector(sectorPosition);
        }
        
        public void GenerateSectors(Vector2Int filamentPosition, Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                GenerateSector(filamentPosition, sectorPosition);
            }
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
        
        public void SaveSectors(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                SaveSector(sectorPosition);
            }
        }

        public void SaveSectors()
        {
            foreach (Vector2Int sectorPosition in loadedSectors.Keys)
            {
                SaveSector(sectorPosition);
            }
        }
        #endregion

        #region Loading
        public bool IsSectorLoaded(Vector2Int sectorPosition)
        {
            return loadedSectors.ContainsKey(sectorPosition);
        }

        public bool AreSectorsLoaded(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                if (!IsSectorLoaded(sectorPosition))
                {
                    return false;
                }
            }
            return true;
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

        public void LoadSectors(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                LoadSector(sectorPosition);
            }
        }

        public void UnloadSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already unloaded!");
            }

            loadedSectors.Remove(sectorPosition);
        }
        
        public void UnloadSectors(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                UnloadSector(sectorPosition);
            }
        }
        
        public void UnloadSectors()
        {
            foreach (Vector2Int sectorPosition in loadedSectors.Keys)
            {
                UnloadSector(sectorPosition);
            }
        }
        #endregion

        #region Spawning
        public void SpawnSector(Vector2Int sectorPosition)
        {
            GetSector(sectorPosition).Spawn();
        }

        public void SpawnSectors(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                SpawnSector(sectorPosition);
            }
        }

        public void DespawnSector(Vector2Int sectorPosition)
        {
            GetSector(sectorPosition).Despawn();
        }
        
        public void DespawnSectors(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                DespawnSector(sectorPosition);
            }
        }
        
        public void DespawnSectors()
        {
            foreach (Vector2Int sectorPosition in loadedSectors.Keys)
            {
                DespawnSector(sectorPosition);
            }
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

        public void DeleteSectors(Vector2Int[] sectorPositions)
        {
            foreach (Vector2Int sectorPosition in sectorPositions)
            {
                DeleteSector(sectorPosition);
            }
        }
        
        public void DeleteSectors()
        {
            foreach (Vector2Int sectorPosition in loadedSectors.Keys)
            {
                DeleteSector(sectorPosition);
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

        public Region.Region[] GetRegions(Vector2Int[] regionPositions)
        {
            Region.Region[] regions = new Region.Region[regionPositions.Length];
            for (int i = 0; i < regionPositions.Length; i++)
            {
                regions[i] = GetRegion(regionPositions[i]);
            }
            return regions;
        }

        public Region.Region[] GetRegions()
        {
            return loadedRegions.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsRegionGenerated(Vector2Int regionPosition)
        {
            string path = $"{Application.dataPath}/Data/Universe/Regions/{regionPosition.x}.{regionPosition.y}.json";
            return File.Exists(path);
        }

        public bool AreRegionsGenerated(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                if (!IsRegionGenerated(regionPosition))
                {
                    return false;
                }
            }
            return true;
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

            loadedRegions.Add(regionPosition, new Region.Region(sectorPosition, regionPosition));
            SaveRegion(regionPosition);
        }
        
        public void GenerateRegions(Vector2Int sectorPosition, Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                GenerateRegion(sectorPosition, regionPosition);
            }
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
        
        public void SaveRegions(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                SaveRegion(regionPosition);
            }
        }

        public void SaveRegions()
        {
            foreach (Vector2Int regionPosition in loadedRegions.Keys)
            {
                SaveRegion(regionPosition);
            }
        }
        #endregion

        #region Loading
        public bool IsRegionLoaded(Vector2Int regionPosition)
        {
            return loadedRegions.ContainsKey(regionPosition);
        }

        public bool AreRegionsLoaded(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                if (!IsRegionLoaded(regionPosition))
                {
                    return false;
                }
            }
            return true;
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

        public void LoadRegions(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                LoadRegion(regionPosition);
            }
        }

        public void UnloadRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already unloaded!");
            }

            loadedRegions.Remove(regionPosition);
        }
        
        public void UnloadRegions(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                UnloadRegion(regionPosition);
            }
        }
        
        public void UnloadRegions()
        {
            foreach (Vector2Int regionPosition in loadedRegions.Keys)
            {
                UnloadRegion(regionPosition);
            }
        }
        #endregion

        #region Spawning
        public void SpawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Spawn();
        }

        public void SpawnRegions(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                SpawnRegion(regionPosition);
            }
        }

        public void DespawnRegion(Vector2Int regionPosition)
        {
            GetRegion(regionPosition).Despawn();
        }

        public void DespawnRegions(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                DespawnRegion(regionPosition);
            }
        }
        
        public void DespawnRegions()
        {
            foreach (Vector2Int regionPosition in loadedRegions.Keys)
            {
                DespawnRegion(regionPosition);
            }
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

        public void DeleteRegions(Vector2Int[] regionPositions)
        {
            foreach (Vector2Int regionPosition in regionPositions)
            {
                DeleteRegion(regionPosition);
            }
        }
        
        public void DeleteRegions()
        {
            foreach (Vector2Int regionPosition in loadedRegions.Keys)
            {
                DeleteRegion(regionPosition);
            }
        }
        #endregion

        #endregion
    }
}