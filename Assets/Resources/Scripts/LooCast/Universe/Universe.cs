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
    using LooCast.Util;
    using LooCast.Test;

    [Serializable]
    public class Universe
    {
        #region Classes

        #endregion

        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            public int seed;
            public int size;
            
            public Void.Void.GenerationSettings voidGenerationSettings;
            public Filament.Filament.GenerationSettings filamentGenerationSettings;
            public Sector.Sector.GenerationSettings sectorGenerationSettings;
            public Region.Region.GenerationSettings regionGenerationSettings;
        }
        #endregion

        public static Universe Instance
        {
            get
            {
                return instance;
            }

            private set
            {
                if (value != null)
                {
                    instance = value;
                    if (!instance.initialized)
                    {
                        instance.Initialize();
                    }
                }
                else
                {
                    instance.Terminate();
                    instance = value;
                }
            }
        }
        private static Universe instance;

        public GenerationSettings UniverseGenerationSettings => generationSettings;
        public Void.Void.GenerationSettings VoidGenerationSettings => generationSettings.voidGenerationSettings;
        public Filament.Filament.GenerationSettings FilamentGenerationSettings => generationSettings.filamentGenerationSettings;
        public Sector.Sector.GenerationSettings SectorGenerationSettings => generationSettings.sectorGenerationSettings;
        public Region.Region.GenerationSettings RegionGenerationSettings => generationSettings.regionGenerationSettings;
        public Texture2D Map
        {
            get
            {
                return map;
            }

            set
            {
                map = value;
            }
        }

        [SerializeField] private GenerationSettings generationSettings;
        [SerializeField] private FastNoiseLite universeNoiseGenerator;
        [SerializeField] private FastNoiseLite universeDomainWarper;
        [SerializeField] private FastNoiseLite filamentNoiseGenerator;
        [SerializeField] private FastNoiseLite filamentDomainWarper;
        [SerializeField] private FastNoiseLite sectorNoiseGenerator;
        [SerializeField] private FastNoiseLite sectorDomainWarper;
        [SerializeField] private FastNoiseLite regionNoiseGenerator;
        [SerializeField] private FastNoiseLite regionDomainWarper;

        private bool initialized = false;

        private Dictionary<Vector2Int, Void.Void> loadedVoids;
        private Dictionary<Vector2Int, Filament.Filament> loadedFilaments;
        private Dictionary<Vector2Int, Sector.Sector> loadedSectors;
        private Dictionary<Vector2Int, Region.Region> loadedRegions;
        private Texture2D map;

        private Universe(GenerationSettings generationSettings)
        {
            this.generationSettings = generationSettings;

            #region Noise Generator & Domain Warper Creation

            #region Universe

            #region Noise Generator Creation
            universeNoiseGenerator = new FastNoiseLite();

            //General
            universeNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.Cellular);
            universeNoiseGenerator.SetSeed(generationSettings.seed);
            universeNoiseGenerator.SetFrequency(0.04f);

            //Fractal
            universeNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            universeNoiseGenerator.SetFractalOctaves(3);
            universeNoiseGenerator.SetFractalLacunarity(2.0f);
            universeNoiseGenerator.SetFractalGain(0.5f);
            universeNoiseGenerator.SetFractalWeightedStrength(1.0f);

            //Cellular
            universeNoiseGenerator.SetCellularDistanceFunction(FastNoiseLite.CellularDistanceFunction.EuclideanSq);
            universeNoiseGenerator.SetCellularReturnType(FastNoiseLite.CellularReturnType.Distance);
            universeNoiseGenerator.SetCellularJitter(1.0f);
            #endregion

            #region Domain Warper Creation
            universeDomainWarper = new FastNoiseLite();

            //General
            universeDomainWarper.SetSeed(generationSettings.seed);
            universeDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            universeDomainWarper.SetDomainWarpAmp(30.0f);
            universeDomainWarper.SetFrequency(0.01f);

            //Fractal
            universeDomainWarper.SetFractalType(FastNoiseLite.FractalType.DomainWarpProgressive);
            universeDomainWarper.SetFractalOctaves(5);
            universeDomainWarper.SetFractalLacunarity(2.0f);
            universeDomainWarper.SetFractalGain(0.5f);
            #endregion

            #endregion

            #region Filament

            #region Noise Generator Creation
            filamentNoiseGenerator = new FastNoiseLite();

            //General
            filamentNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.Cellular);
            filamentNoiseGenerator.SetSeed(generationSettings.seed);
            filamentNoiseGenerator.SetFrequency(0.04f);

            //Fractal
            filamentNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            filamentNoiseGenerator.SetFractalOctaves(3);
            filamentNoiseGenerator.SetFractalLacunarity(2.0f);
            filamentNoiseGenerator.SetFractalGain(0.5f);
            filamentNoiseGenerator.SetFractalWeightedStrength(1.0f);

            //Cellular
            filamentNoiseGenerator.SetCellularDistanceFunction(FastNoiseLite.CellularDistanceFunction.EuclideanSq);
            filamentNoiseGenerator.SetCellularReturnType(FastNoiseLite.CellularReturnType.Distance);
            filamentNoiseGenerator.SetCellularJitter(1.0f);
            #endregion

            #region Domain Warper Creation
            filamentDomainWarper = new FastNoiseLite();

            //General
            filamentDomainWarper.SetSeed(generationSettings.seed);
            filamentDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            filamentDomainWarper.SetDomainWarpAmp(30.0f);
            filamentDomainWarper.SetFrequency(0.01f);

            //Fractal
            filamentDomainWarper.SetFractalType(FastNoiseLite.FractalType.DomainWarpProgressive);
            filamentDomainWarper.SetFractalOctaves(5);
            filamentDomainWarper.SetFractalLacunarity(2.0f);
            filamentDomainWarper.SetFractalGain(0.5f);
            #endregion

            #endregion

            #region Sector

            #region Noise Generator Creation
            sectorNoiseGenerator = new FastNoiseLite();

            //General
            sectorNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.Cellular);
            sectorNoiseGenerator.SetSeed(generationSettings.seed);
            sectorNoiseGenerator.SetFrequency(0.04f);

            //Fractal
            sectorNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            sectorNoiseGenerator.SetFractalOctaves(3);
            sectorNoiseGenerator.SetFractalLacunarity(2.0f);
            sectorNoiseGenerator.SetFractalGain(0.5f);
            sectorNoiseGenerator.SetFractalWeightedStrength(1.0f);

            //Cellular
            sectorNoiseGenerator.SetCellularDistanceFunction(FastNoiseLite.CellularDistanceFunction.EuclideanSq);
            sectorNoiseGenerator.SetCellularReturnType(FastNoiseLite.CellularReturnType.Distance);
            sectorNoiseGenerator.SetCellularJitter(1.0f);
            #endregion

            #region Domain Warper Creation
            sectorDomainWarper = new FastNoiseLite();

            //General
            sectorDomainWarper.SetSeed(generationSettings.seed);
            sectorDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            sectorDomainWarper.SetDomainWarpAmp(30.0f);
            sectorDomainWarper.SetFrequency(0.01f);

            //Fractal
            sectorDomainWarper.SetFractalType(FastNoiseLite.FractalType.DomainWarpProgressive);
            sectorDomainWarper.SetFractalOctaves(5);
            sectorDomainWarper.SetFractalLacunarity(2.0f);
            sectorDomainWarper.SetFractalGain(0.5f);
            #endregion

            #endregion

            #region Region

            #region Noise Generator Creation
            regionNoiseGenerator = new FastNoiseLite();

            //General
            regionNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.Cellular);
            regionNoiseGenerator.SetSeed(generationSettings.seed);
            regionNoiseGenerator.SetFrequency(0.04f);

            //Fractal
            regionNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            regionNoiseGenerator.SetFractalOctaves(3);
            regionNoiseGenerator.SetFractalLacunarity(2.0f);
            regionNoiseGenerator.SetFractalGain(0.5f);
            regionNoiseGenerator.SetFractalWeightedStrength(1.0f);

            //Cellular
            regionNoiseGenerator.SetCellularDistanceFunction(FastNoiseLite.CellularDistanceFunction.EuclideanSq);
            regionNoiseGenerator.SetCellularReturnType(FastNoiseLite.CellularReturnType.Distance);
            regionNoiseGenerator.SetCellularJitter(1.0f);
            #endregion

            #region Domain Warper Creation
            regionDomainWarper = new FastNoiseLite();

            //General
            regionDomainWarper.SetSeed(generationSettings.seed);
            regionDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            regionDomainWarper.SetDomainWarpAmp(30.0f);
            regionDomainWarper.SetFrequency(0.01f);

            //Fractal
            regionDomainWarper.SetFractalType(FastNoiseLite.FractalType.DomainWarpProgressive);
            regionDomainWarper.SetFractalOctaves(5);
            regionDomainWarper.SetFractalLacunarity(2.0f);
            regionDomainWarper.SetFractalGain(0.5f);
            #endregion

            #endregion

            #endregion

            #region Main Generation
            SeededRandom prng = new SeededRandom(generationSettings.seed);
            for (int x = 0; x < generationSettings.voidGenerationSettings.amount; x++)
            {
                for (int y = 0; y < generationSettings.voidGenerationSettings.amount; y++)
                {
                    Vector2Int voidPosition = new Vector2Int(x, y);
                    Vector2 normalizedVoidPositionOffset = new Vector2(prng.Range(-0.5f, 0.5f), prng.Range(-0.5f, 0.5f));
                    GenerateVoid(voidPosition, normalizedVoidPositionOffset);
                }
            }

            Color[] noiseColorMap = new Color[generationSettings.size * generationSettings.size];
            for (int y = 0; y < generationSettings.size; y++)
            {
                for (int x = 0; x < generationSettings.size; x++)
                {
                    float sampleX = x;
                    float sampleY = y;
                    domainWarper.DomainWarp(ref sampleX, ref sampleY);
                    float noiseValue = noiseGenerator.GetNoise(sampleX, sampleY);
                    noiseValue = noiseValue.Map(-1, 1, -0.375f, 1.375f);
                    noiseColorMap[y * generationSettings.size + x] = new Color(noiseValue, noiseValue, noiseValue, 1.0f);
                }
            }

            map = TextureUtil.TextureFromColorMap(noiseColorMap, generationSettings.size, generationSettings.size);
            #endregion
        }

        #region Universe

        #region Initialization
        private void Initialize()
        {
            loadedVoids = new Dictionary<Vector2Int, Void.Void>();
            loadedFilaments = new Dictionary<Vector2Int, Filament.Filament>();
            loadedSectors = new Dictionary<Vector2Int, Sector.Sector>();
            loadedRegions = new Dictionary<Vector2Int, Region.Region>();

            initialized = true;
        }
        #endregion

        #region Termination
        private void Terminate()
        {

        }
        #endregion

        #region Utility
        public static float GetMapPixelValue(Vector2Int pixelPosition)
        {
            if (!IsUniverseLoaded())
            {
                throw new Exception("Universe is not loaded!");
            }

            return Instance.map.GetPixel(pixelPosition.x, pixelPosition.y).grayscale;
        }
        #endregion

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
            Instance = universe;
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
            using StreamWriter streamWriter = new StreamWriter(path);
            streamWriter.Write(json);

            path = $"{Application.dataPath}/Data/Universe/Map.png";
            byte[] mapData = ImageConversion.EncodeToPNG(Instance.map);
            using BinaryWriter binaryWriter = new BinaryWriter(File.OpenWrite(path));
            binaryWriter.Write(mapData);

            Instance.SaveFilaments();
            Instance.SaveSectors();
            Instance.SaveRegions();
        }
        #endregion

        #region Loading
        public static bool IsUniverseLoaded()
        {
            return Instance != null;
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
            Instance = JsonUtility.FromJson<Universe>(json);

            path = $"{Application.dataPath}/Data/Universe/Map.png";
            byte[] mapData = File.ReadAllBytes(path);
            ImageConversion.LoadImage(Instance.map, mapData);
        }

        public static void UnloadUniverse()
        {
            if (!IsUniverseLoaded())
            {
                throw new Exception("Universe is already unloaded!");
            }

            Instance = null;
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

    	    Void.Void @void = new Void.Void(voidPosition, normalizedVoidPositionOffset);
            loadedVoids.Add(voidPosition, @void);
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

        private void SaveVoids(Void.Void[] voids)
        {
            foreach (Void.Void @void in voids)
            {
                SaveVoid(@void);
            }
        }

        private void SaveVoid(Vector2Int voidPosition)
        {
            if (!IsVoidLoaded(voidPosition))
            {
                throw new Exception("Void is not loaded!");
            }

            Void.Void @void = GetVoid(voidPosition);
            SaveVoid(@void);
        }

        private void SaveVoids(Vector2Int[] voidPositions)
        {
            foreach (Vector2Int voidPosition in voidPositions)
            {
                SaveVoid(voidPosition);
            }
        }

        private void SaveVoids()
        {
            foreach (Vector2Int voidPosition in loadedVoids.Keys.ToArray())
            {
                SaveFilament(voidPosition);
            }
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
            foreach (Void.Void @void in loadedVoids.Values.ToArray())
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

            Filament.Filament filament = new Filament.Filament(filamentPosition);
            loadedFilaments.Add(filamentPosition, filament);
            SaveFilament(filament);
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
        public void SaveFilament(Filament.Filament filament)
        {
            string path = $"{Application.dataPath}/Data/Universe/Filaments/{filament.FilamentPosition.x}.{filament.FilamentPosition.y}.json";
            string json = JsonUtility.ToJson(filament, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);

            path = $"{Application.dataPath}/Data/Universe/Filaments/{filament.FilamentPosition.x}.{filament.FilamentPosition.y}_Map.png";
            byte[] mapData = ImageConversion.EncodeToPNG(filament.Map);
            using BinaryWriter binaryWriter = new BinaryWriter(File.OpenWrite(path));
            binaryWriter.Write(mapData);
        }

        public void SaveFilaments(Filament.Filament[] filaments)
        {
            foreach (Filament.Filament filament in filaments)
            {
                SaveFilament(filament);
            }
        }

        public void SaveFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is not loaded!");
            }

            Filament.Filament filament = GetFilament(filamentPosition);
            SaveFilament(filament);
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
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys.ToArray())
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
            Filament.Filament filament = JsonUtility.FromJson<Filament.Filament>(json);
            loadedFilaments.Add(filamentPosition, filament);

            path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}_Map.png";
            byte[] mapData = File.ReadAllBytes(path);
            ImageConversion.LoadImage(filament.Map, mapData);
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
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys.ToArray())
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
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys.ToArray())
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

                path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}_Map.png";
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
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys.ToArray())
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

            Sector.Sector sector = new Sector.Sector(filamentPosition, sectorPosition);
            loadedSectors.Add(sectorPosition, sector);
            SaveSector(sector);
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
        public void SaveSector(Sector.Sector sector)
        {
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sector.SectorPosition.x}.{sector.SectorPosition.y}.json";
            string json = JsonUtility.ToJson(sector, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveSector(Sector.Sector[] sectors)
        {
            foreach (Sector.Sector sector in sectors)
            {
                SaveSector(sector);
            }
        }
        
        public void SaveSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is not loaded!");
            }

            Sector.Sector sector = GetSector(sectorPosition);
            SaveSector(sector);
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
            foreach (Vector2Int sectorPosition in loadedSectors.Keys.ToArray())
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
            foreach (Vector2Int sectorPosition in loadedSectors.Keys.ToArray())
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
            foreach (Vector2Int sectorPosition in loadedSectors.Keys.ToArray())
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
            foreach (Vector2Int sectorPosition in loadedSectors.Keys.ToArray())
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

            Region.Region region = new Region.Region(sectorPosition, regionPosition);
            loadedRegions.Add(regionPosition, region);
            SaveRegion(region);
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
        public void SaveRegion(Region.Region region)
        {
            string path = $"{Application.dataPath}/Data/Universe/Regions/{region.RegionPosition.x}.{region.RegionPosition.y}.json";
            string json = JsonUtility.ToJson(region, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveRegions(Region.Region[] regions)
        {
            foreach (Region.Region region in regions)
            {
                SaveRegion(region);
            }
        }
        
        public void SaveRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is not loaded!");
            }

            Region.Region region = GetRegion(regionPosition);
            SaveRegion(region);
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
            foreach (Vector2Int regionPosition in loadedRegions.Keys.ToArray())
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
            foreach (Vector2Int regionPosition in loadedRegions.Keys.ToArray())
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
            foreach (Vector2Int regionPosition in loadedRegions.Keys.ToArray())
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
            foreach (Vector2Int regionPosition in loadedRegions.Keys.ToArray())
            {
                DeleteRegion(regionPosition);
            }
        }
        #endregion

        #endregion
    }
}