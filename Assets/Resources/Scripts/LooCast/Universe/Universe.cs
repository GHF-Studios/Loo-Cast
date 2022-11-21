using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;
using System.Linq;

namespace LooCast.Universe
{
    using LooCast.Game;
    using Util;
    using Util.Collections.Generic;

    [Serializable]
    public class Universe
    {
        #region Classes
        [Serializable]
        public class Filament
        {
            #region Classes
            [Serializable]
            public class Chunk
            {
                public int Size => size;
                public Vector2Int ChunkPosition => chunkPosition;
                public SerializableMap2D<float?> ElectronDensityMap => electronDensityMap;
                public SerializableMap2D<float?> PositronDensityMap => positronDensityMap;
                public SerializableMap2D<float?> ProtonDensityMap => protonDensityMap;
                public SerializableMap2D<float?> AntiProtonDensityMap => antiProtonDensityMap;
                public SerializableMap2D<float?> NeutronDensityMap => neutronDensityMap;
                public SerializableMap2D<float?> AntiNeutronDensityMap => antiNeutronDensityMap;

                public Chunk(Universe universe, Filament filament, Vector2Int chunkPosition)
                {
                    GenerationSettings generationSettings = universe.FilamentGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;

                    Vector2Int filamentPosition = filament.FilamentPosition;

                    #region Electron Density Map Generation
                    electronDensityMap = new SerializableMap2D<float?>(generationSettings.ChunkSize, generationSettings.ChunkSize);

                    for (int y = 0; y < generationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < generationSettings.ChunkSize; x++)
                        {
                            #region Filament Noise Sampling
                            float filamentOffsetX = -((filamentPosition.x * generationSettings.Size) + chunkPosition.x * generationSettings.ChunkSize);
                            float filamentOffsetY = -((filamentPosition.y * generationSettings.Size) + chunkPosition.y * generationSettings.ChunkSize);

                            float filamentSampleX = x + filamentOffsetX;
                            float filamentSampleY = y + filamentOffsetY;

                            float electronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Universe Noise Sampling
                            float universeOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                            float universeOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                            float universeSampleX = filamentPosition.x + universeOffsetX;
                            float universeSampleY = filamentPosition.y + universeOffsetY;

                            float universeNoiseValue = universe.SampleNoise(universeSampleX, universeSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Total Density Evaluation
                            universeNoiseValue = universeNoiseValue.Map(0, 1, -1, 1);
                            float totalElectronDensity = electronDensity * (1 + (generationSettings.UniverseNoiseInfluence * universeNoiseValue));
                            #endregion

                            electronDensityMap.SetValue(x, y, totalElectronDensity);
                        }
                    }
                    #endregion
                }

                [SerializeField] private int size;
                [SerializeField] private Vector2Int chunkPosition;
                [SerializeField] private SerializableMap2D<float?> electronDensityMap;
                [SerializeField] private SerializableMap2D<float?> positronDensityMap;
                [SerializeField] private SerializableMap2D<float?> protonDensityMap;
                [SerializeField] private SerializableMap2D<float?> antiProtonDensityMap;
                [SerializeField] private SerializableMap2D<float?> neutronDensityMap;
                [SerializeField] private SerializableMap2D<float?> antiNeutronDensityMap;
            }
            #endregion

            #region Structs
            [Serializable]
            public struct GenerationSettings
            {
                public GameObject Prefab;
                public int ChunkSize;
                public int ChunkAmount;
                public int Size
                {
                    get
                    {
                        return ChunkAmount * ChunkSize;
                    }
                }
                public float MapFromMin;
                public float MapFromMax;
                public float MapToMin;
                public float MapToMax;
                public float UniverseNoiseInfluence;
                public float Power;
                public float Amplitude;

                [Header("FNL Noise General Settings")]
                public FastNoiseLite.NoiseType NoiseType;
                public float Frequency;

                [Header("FNL Noise Fractal Settings")]
                public FastNoiseLite.FractalType FractalType;
                public int FractalOctaves;
                public float FractalLacunarity;
                public float FractalGain;
                public float FractalWeightedStrength;

                [Header("FNL Noise Cellular Settings")]
                public FastNoiseLite.CellularDistanceFunction CellularDistanceFunction;
                public FastNoiseLite.CellularReturnType CellularReturnType;
                public float CellularJitter;

                [Header("FNL Domain Warp General Settings")]
                public FastNoiseLite.DomainWarpType DomainWarpType;
                public float DomainWarpAmplitude;
                public float DomainWarpFrequency;

                [Header("FNL Domain Warp Fractal Settings")]
                public FastNoiseLite.FractalType DomainWarpFractalType;
                public int DomainWarpFractalOctaves;
                public float DomainWarpFractalLacunarity;
                public float DomainWarpFractalGain;
            }
            #endregion

            public Vector2Int FilamentPosition => filamentPosition;
            public SerializableMap2D<Chunk> ChunkMap => chunkMap;

            [SerializeField] private Vector2Int filamentPosition;
            [SerializeField] private SerializableMap2D<Chunk> chunkMap;

            public Filament(Vector2Int filamentPosition)
            {
                this.filamentPosition = filamentPosition;
            }

            public float SampleNoise(Universe universe, float sampleX, float sampleY)
            {
                #region Sampling
                universe.FilamentDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                float noiseValue = universe.FilamentNoiseGenerator.GetNoise(sampleX, sampleY);
                #endregion

                #region Processing
                GenerationSettings generationSettings = universe.FilamentGenerationSettings;
                noiseValue = noiseValue.Map(generationSettings.MapFromMin, generationSettings.MapFromMax, generationSettings.MapToMin, generationSettings.MapToMax);
                noiseValue = Mathf.Pow(noiseValue, generationSettings.Power);
                noiseValue *= generationSettings.Amplitude;
                #endregion

                return noiseValue;
            }
        }

        [Serializable]
        public class Sector
        {
            #region Classes
            [Serializable]
            public class Chunk
            {
                public int Size => size;
                public Vector2Int ChunkPosition => chunkPosition;
                public SerializableMap2D<float?> SolidParticleDensityMap => solidParticleDensityMap;
                public SerializableMap2D<float?> LiquidParticleDensityMap => liquidParticleDensityMap;
                public SerializableMap2D<float?> GasParticleDensityMap => gasParticleDensityMap;
                public SerializableMap2D<float?> PlasmaParticleDensityMap => plasmaParticleDensityMap;

                public Chunk(Universe universe, Sector sector, Vector2Int chunkPosition)
                {
                    GenerationSettings generationSettings = universe.SectorGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;

                    Vector2Int sectorPosition = sector.SectorPosition;

                    #region Solid Particle Density Map Generation
                    solidParticleDensityMap = new SerializableMap2D<float?>(generationSettings.ChunkSize, generationSettings.ChunkSize);

                    for (int y = 0; y < generationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < generationSettings.ChunkSize; x++)
                        {
                            #region Sector Noise Sampling
                            float sectorOffsetX = -((sectorPosition.x * generationSettings.Size) + chunkPosition.x * generationSettings.ChunkSize);
                            float sectorOffsetY = -((sectorPosition.y * generationSettings.Size) + chunkPosition.y * generationSettings.ChunkSize);

                            float sectorSampleX = x + sectorOffsetX;
                            float sectorSampleY = y + sectorOffsetY;

                            float solidParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Sector Noise Sampling
                            float filamentOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                            float filamentOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                            float filamentSampleX = sectorPosition.x + filamentOffsetX;
                            float filamentSampleY = sectorPosition.y + filamentOffsetY;

                            float filamentNoiseValue = universe.SampleNoise(filamentSampleX, filamentSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Total Density Evaluation
                            filamentNoiseValue = filamentNoiseValue.Map(0, 1, -1, 1);
                            float totalSolidParticleDensity = solidParticleDensity * (1 + (generationSettings.FilamentNoiseInfluence * filamentNoiseValue));
                            #endregion

                            solidParticleDensityMap.SetValue(x, y, totalSolidParticleDensity);
                        }
                    }
                    #endregion
                }

                [SerializeField] private int size;
                [SerializeField] private Vector2Int chunkPosition;
                [SerializeField] private SerializableMap2D<float?> solidParticleDensityMap;
                [SerializeField] private SerializableMap2D<float?> liquidParticleDensityMap;
                [SerializeField] private SerializableMap2D<float?> gasParticleDensityMap;
                [SerializeField] private SerializableMap2D<float?> plasmaParticleDensityMap;
            }
            #endregion

            #region Structs
            [Serializable]
            public struct GenerationSettings
            {
                public GameObject Prefab;
                public int ChunkSize;
                public int ChunkAmount;
                public int Size
                {
                    get
                    {
                        return ChunkAmount * ChunkSize;
                    }
                }
                public float MapFromMin;
                public float MapFromMax;
                public float MapToMin;
                public float MapToMax;
                public float FilamentNoiseInfluence;
                public float Power;
                public float Amplitude;

                [Header("FNL Noise General Settings")]
                public FastNoiseLite.NoiseType NoiseType;
                public float Frequency;

                [Header("FNL Noise Fractal Settings")]
                public FastNoiseLite.FractalType FractalType;
                public int FractalOctaves;
                public float FractalLacunarity;
                public float FractalGain;
                public float FractalWeightedStrength;

                [Header("FNL Domain Warp General Settings")]
                public FastNoiseLite.DomainWarpType DomainWarpType;
                public float DomainWarpAmplitude;
                public float DomainWarpFrequency;

                [Header("FNL Domain Warp Fractal Settings")]
                public FastNoiseLite.FractalType DomainWarpFractalType;
                public int DomainWarpFractalOctaves;
                public float DomainWarpFractalLacunarity;
                public float DomainWarpFractalGain;
            }
            #endregion

            public Vector2Int FilamentPosition => filamentPosition;
            public Vector2Int SectorPosition => sectorPosition;
            public SerializableMap2D<Chunk> ChunkMap => chunkMap;

            [SerializeField] private Vector2Int filamentPosition;
            [SerializeField] private Vector2Int sectorPosition;
            [SerializeField] private SerializableMap2D<Chunk> chunkMap;

            public Sector(Vector2Int filamentPosition, Vector2Int sectorPosition)
            {
                this.filamentPosition = filamentPosition;
                this.sectorPosition = sectorPosition;
            }

            public float SampleNoise(Universe universe, float sampleX, float sampleY)
            {
                #region Sampling
                universe.SectorDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                float noiseValue = universe.SectorNoiseGenerator.GetNoise(sampleX, sampleY);
                #endregion

                #region Processing
                GenerationSettings generationSettings = universe.SectorGenerationSettings;
                noiseValue = noiseValue.Map(generationSettings.MapFromMin, generationSettings.MapFromMax, generationSettings.MapToMin, generationSettings.MapToMax);
                noiseValue = Mathf.Pow(noiseValue, generationSettings.Power);
                noiseValue *= generationSettings.Amplitude;
                #endregion

                return noiseValue;
            }
        }

        [Serializable]
        public class Region
        {
            #region Classes
            [Serializable]
            public class Chunk
            {
                public int Size => size;
                public Vector2Int ChunkPosition => chunkPosition;
                public SerializableMap2D<float?> MatterDensityMap => matterDensityMap;
                public SerializableMap2D<float?> AntiMatterDensityMap => antiMatterDensityMap;

                public Chunk(Universe universe, Region region, Vector2Int chunkPosition)
                {
                    GenerationSettings generationSettings = universe.RegionGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;

                    Vector2Int regionPosition = region.RegionPosition;

                    #region Matter Density Map Generation
                    matterDensityMap = new SerializableMap2D<float?>(generationSettings.ChunkSize, generationSettings.ChunkSize);

                    for (int y = 0; y < generationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < generationSettings.ChunkSize; x++)
                        {
                            #region Region Noise Sampling
                            float regionOffsetX = -((regionPosition.x * generationSettings.Size) + chunkPosition.x * generationSettings.ChunkSize);
                            float regionOffsetY = -((regionPosition.y * generationSettings.Size) + chunkPosition.y * generationSettings.ChunkSize);

                            float regionSampleX = x + regionOffsetX;
                            float regionSampleY = y + regionOffsetY;

                            float matterDensity = region.SampleNoise(universe, regionSampleX, regionSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Sector Noise Sampling
                            float sectorOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                            float sectorOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                            float sectorSampleX = regionPosition.x + sectorOffsetX;
                            float sectorSampleY = regionPosition.y + sectorOffsetY;

                            float sectorNoiseValue = universe.SampleNoise(sectorSampleX, sectorSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Total Density Evaluation
                            sectorNoiseValue = sectorNoiseValue.Map(0, 1, -1, 1);
                            float totalMatterDensity = matterDensity * (1 + (generationSettings.SectorNoiseInfluence * sectorNoiseValue));
                            #endregion

                            matterDensityMap.SetValue(x, y, totalMatterDensity);
                        }
                    }
                    #endregion
                }

                [SerializeField] private int size;
                [SerializeField] private Vector2Int chunkPosition;
                [SerializeField] private SerializableMap2D<float?> matterDensityMap;
                [SerializeField] private SerializableMap2D<float?> antiMatterDensityMap;
            }
            #endregion

            #region Structs
            [Serializable]
            public struct GenerationSettings
            {
                public GameObject Prefab;
                public int ChunkSize;
                public int ChunkAmount;
                public int Size
                {
                    get
                    {
                        return ChunkAmount * ChunkSize;
                    }
                }
                public float MapFromMin;
                public float MapFromMax;
                public float MapToMin;
                public float MapToMax;
                public float SectorNoiseInfluence;
                public float Power;
                public float Amplitude;

                [Header("FNL Noise General Settings")]
                public FastNoiseLite.NoiseType NoiseType;
                public float Frequency;

                [Header("FNL Noise Fractal Settings")]
                public FastNoiseLite.FractalType FractalType;
                public int FractalOctaves;
                public float FractalLacunarity;
                public float FractalGain;
                public float FractalWeightedStrength;

                [Header("FNL Domain Warp General Settings")]
                public FastNoiseLite.DomainWarpType DomainWarpType;
                public float DomainWarpAmplitude;
                public float DomainWarpFrequency;

                [Header("FNL Domain Warp Fractal Settings")]
                public FastNoiseLite.FractalType DomainWarpFractalType;
                public int DomainWarpFractalOctaves;
                public float DomainWarpFractalLacunarity;
                public float DomainWarpFractalGain;
            }
            #endregion

            public Vector2Int SectorPosition => sectorPosition;
            public Vector2Int RegionPosition => regionPosition;
            public SerializableMap2D<Chunk> ChunkMap => chunkMap;

            [SerializeField] private Vector2Int sectorPosition;
            [SerializeField] private Vector2Int regionPosition;
            [SerializeField] private SerializableMap2D<Chunk> chunkMap;

            public Region(Vector2Int sectorPosition, Vector2Int regionPosition)
            {
                this.sectorPosition = sectorPosition;
                this.regionPosition = regionPosition;
            }

            public float SampleNoise(Universe universe, float sampleX, float sampleY)
            {
                #region Sampling
                universe.RegionDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                float noiseValue = universe.RegionNoiseGenerator.GetNoise(sampleX, sampleY);
                #endregion

                #region Processing
                GenerationSettings generationSettings = universe.RegionGenerationSettings;
                noiseValue = noiseValue.Map(generationSettings.MapFromMin, generationSettings.MapFromMax, generationSettings.MapToMin, generationSettings.MapToMax);
                noiseValue = Mathf.Pow(noiseValue, generationSettings.Power);
                noiseValue *= generationSettings.Amplitude;
                #endregion

                return noiseValue;
            }

            public Vector2Int GetFilamentPosition(Universe universe, Vector2Int sectorPosition)
            {
                return universe.GetSector(sectorPosition).FilamentPosition;
            }
        }
        #endregion

        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            [Header("Main Settings")]
            public int Seed;
            public int Size;
            public float MapFromMin;
            public float MapFromMax;
            public float MapToMin;
            public float MapToMax;
            public float Power;
            public float Amplitude;

            [Header("FNL Noise General Settings")]
            public FastNoiseLite.NoiseType NoiseType;
            public float Frequency;

            [Header("FNL Noise Fractal Settings")]
            public FastNoiseLite.FractalType FractalType;
            public int FractalOctaves;
            public float FractalLacunarity;
            public float FractalGain;
            public float FractalWeightedStrength;

            [Header("FNL Noise Cellular Settings")]
            public FastNoiseLite.CellularDistanceFunction CellularDistanceFunction;
            public FastNoiseLite.CellularReturnType CellularReturnType;
            public float CellularJitter;

            [Header("FNL Domain Warp General Settings")]
            public FastNoiseLite.DomainWarpType DomainWarpType;
            public float DomainWarpAmplitude;
            public float DomainWarpFrequency;

            [Header("FNL Domain Warp Fractal Settings")]
            public FastNoiseLite.FractalType DomainWarpFractalType;
            public int DomainWarpFractalOctaves;
            public float DomainWarpFractalLacunarity;
            public float DomainWarpFractalGain;

            public Filament.GenerationSettings FilamentGenerationSettings;
            public Sector.GenerationSettings SectorGenerationSettings;
            public Region.GenerationSettings RegionGenerationSettings;
        }
        #endregion

        #region Static Properties
        public static string DataPath
        {
            get
            {
                return GetDataPath();
            }
        }
        #endregion

        #region Properties
        public bool Initialized => initialized;
        public Texture2D Map
        {
            get
            {
                if (map == null)
                {
                    string path = $"{DataPath}/Map.png";
                    byte[] mapData = File.ReadAllBytes(path);
                    map = new Texture2D(UniverseGenerationSettings.Size, UniverseGenerationSettings.Size);
                    map.filterMode = FilterMode.Point;
                    map.wrapMode = TextureWrapMode.Clamp;
                    ImageConversion.LoadImage(map, mapData);
                }
                return map;
            }

            private set
            {
                map = value;
                string path = $"{DataPath}/Map.png";
                byte[] mapData = ImageConversion.EncodeToPNG(map);
                using BinaryWriter binaryWriter = new BinaryWriter(File.OpenWrite(path));
                binaryWriter.Write(mapData);
            }
        }
        public GenerationSettings UniverseGenerationSettings => generationSettings;
        public Filament.GenerationSettings FilamentGenerationSettings => generationSettings.FilamentGenerationSettings;
        public Sector.GenerationSettings SectorGenerationSettings => generationSettings.SectorGenerationSettings;
        public Region.GenerationSettings RegionGenerationSettings => generationSettings.RegionGenerationSettings;
        public FastNoiseLite UniverseNoiseGenerator => universeNoiseGenerator;
        public FastNoiseLite UniverseDomainWarper => universeDomainWarper;
        public FastNoiseLite FilamentNoiseGenerator => filamentNoiseGenerator;
        public FastNoiseLite FilamentDomainWarper => filamentDomainWarper;
        public FastNoiseLite SectorNoiseGenerator => sectorNoiseGenerator;
        public FastNoiseLite SectorDomainWarper => sectorDomainWarper;
        public FastNoiseLite RegionNoiseGenerator => regionNoiseGenerator;
        public FastNoiseLite RegionDomainWarper => regionDomainWarper;
        public Filament[] LoadedFilaments
        {
            get
            {
                return loadedFilaments.Values.ToArray();
            }
        }
        public Sector[] LoadedSectors
        {
            get
            {
                return loadedSectors.Values.ToArray();
            }
        }
        public Region[] LoadedRegions
        {
            get
            {
                return loadedRegions.Values.ToArray();
            }
        }
        #endregion

        #region Fields
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
        private Texture2D map;
        private Dictionary<Vector2Int, Filament> loadedFilaments;
        private Dictionary<Vector2Int, Sector> loadedSectors;
        private Dictionary<Vector2Int, Region> loadedRegions;
        #endregion

        #region Constructors
        private Universe(GenerationSettings generationSettings)
        {
            this.generationSettings = generationSettings;

            #region Noise Generators & Domain Warpers Creation

            #region Universe

            #region Noise Generator Creation
            universeNoiseGenerator = new FastNoiseLite();

            //General
            universeNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.Cellular);
            universeNoiseGenerator.SetSeed(generationSettings.Seed);
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
            universeDomainWarper.SetSeed(generationSettings.Seed);
            universeDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            universeDomainWarper.SetDomainWarpAmp(20.0f);
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
            filamentNoiseGenerator.SetSeed(generationSettings.Seed);
            filamentNoiseGenerator.SetFrequency(0.02f);

            //Fractal
            filamentNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            filamentNoiseGenerator.SetFractalOctaves(5);
            filamentNoiseGenerator.SetFractalLacunarity(2.0f);
            filamentNoiseGenerator.SetFractalGain(0.5f);
            filamentNoiseGenerator.SetFractalWeightedStrength(0.0f);

            //Cellular
            filamentNoiseGenerator.SetCellularDistanceFunction(FastNoiseLite.CellularDistanceFunction.EuclideanSq);
            filamentNoiseGenerator.SetCellularReturnType(FastNoiseLite.CellularReturnType.Distance);
            filamentNoiseGenerator.SetCellularJitter(1.0f);
            #endregion

            #region Domain Warper Creation
            filamentDomainWarper = new FastNoiseLite();

            //General
            filamentDomainWarper.SetSeed(generationSettings.Seed);
            filamentDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            filamentDomainWarper.SetDomainWarpAmp(20.0f);
            filamentDomainWarper.SetFrequency(0.005f);

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
            sectorNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.OpenSimplex2);
            sectorNoiseGenerator.SetSeed(generationSettings.Seed);
            sectorNoiseGenerator.SetFrequency(0.01f);

            //Fractal
            sectorNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            sectorNoiseGenerator.SetFractalOctaves(5);
            sectorNoiseGenerator.SetFractalLacunarity(2.0f);
            sectorNoiseGenerator.SetFractalGain(0.5f);
            sectorNoiseGenerator.SetFractalWeightedStrength(0.0f);
            #endregion

            #region Domain Warper Creation
            sectorDomainWarper = new FastNoiseLite();

            //General
            sectorDomainWarper.SetSeed(generationSettings.Seed);
            sectorDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            sectorDomainWarper.SetDomainWarpAmp(20.0f);
            sectorDomainWarper.SetFrequency(0.005f);

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
            regionNoiseGenerator.SetNoiseType(FastNoiseLite.NoiseType.OpenSimplex2);
            regionNoiseGenerator.SetSeed(generationSettings.Seed);
            regionNoiseGenerator.SetFrequency(0.005f);

            //Fractal
            regionNoiseGenerator.SetFractalType(FastNoiseLite.FractalType.FBm);
            regionNoiseGenerator.SetFractalOctaves(5);
            regionNoiseGenerator.SetFractalLacunarity(2.0f);
            regionNoiseGenerator.SetFractalGain(0.5f);
            regionNoiseGenerator.SetFractalWeightedStrength(0.0f);
            #endregion

            #region Domain Warper Creation
            regionDomainWarper = new FastNoiseLite();

            //General
            regionDomainWarper.SetSeed(generationSettings.Seed);
            regionDomainWarper.SetDomainWarpType(FastNoiseLite.DomainWarpType.OpenSimplex2);
            regionDomainWarper.SetDomainWarpAmp(20.0f);
            regionDomainWarper.SetFrequency(0.005f);

            //Fractal
            regionDomainWarper.SetFractalType(FastNoiseLite.FractalType.DomainWarpProgressive);
            regionDomainWarper.SetFractalOctaves(5);
            regionDomainWarper.SetFractalLacunarity(2.0f);
            regionDomainWarper.SetFractalGain(0.5f);
            #endregion

            #endregion

            #endregion

            #region Universe Map Generation
            Color[] noiseColorMap = new Color[UniverseGenerationSettings.Size * UniverseGenerationSettings.Size];
            for (int y = 0; y < UniverseGenerationSettings.Size; y++)
            {
                for (int x = 0; x < UniverseGenerationSettings.Size; x++)
                {
                    float sampleX = x;
                    float sampleY = y;

                    float noiseValue = SampleNoise(sampleX, sampleY);

                    noiseColorMap[y * UniverseGenerationSettings.Size + x] = new Color(noiseValue, noiseValue, noiseValue, 1.0f);
                }
            }

            Map = TextureUtil.TextureFromColorMap(noiseColorMap, UniverseGenerationSettings.Size, UniverseGenerationSettings.Size);
            #endregion

            Initialize();
        }
        #endregion

        #region Static Methods
        public static Universe GenerateUniverse(GenerationSettings generationSettings)
        {
            if (!GameManager.Initialized)
            {
                throw new Exception("Cannot generate Universe when GameManager is not initialized!");
            }
            if (GameManager.Instance.CurrentGame == null)
            {
                throw new Exception("Cannot generate Universe when no Game is loaded!");
            }

            Universe universe = new Universe(generationSettings);
            return universe;
        }

        private static string GetDataPath()
        {
            if (!GameManager.Initialized)
            {
                throw new Exception("Cannot get DataPath when GameManager is not initialized!");
            }
            if (GameManager.Instance.CurrentGame == null)
            {
                throw new Exception("Cannot get DataPath when no Game is loaded!");
            }

            return $"{GameManager.Instance.CurrentGame.DataPath}/Universe";
        }
        #endregion

        #region Methods
        public void Initialize()
        {
            if (GameManager.Instance.CurrentGame.CurrentUniverse.Initialized)
            {
                throw new Exception("Cannot initialize Universe when Universe is already initialized!");
            }

            loadedFilaments = new Dictionary<Vector2Int, Filament>();
            loadedSectors = new Dictionary<Vector2Int, Sector>();
            loadedRegions = new Dictionary<Vector2Int, Region>();

            initialized = true;
        }

        public float SampleNoise(float sampleX, float sampleY)
        {
            #region Sampling
            UniverseDomainWarper.DomainWarp(ref sampleX, ref sampleY);
            float noiseValue = UniverseNoiseGenerator.GetNoise(sampleX, sampleY);
            #endregion

            #region Processing
            noiseValue = noiseValue.Map(UniverseGenerationSettings.MapFromMin, UniverseGenerationSettings.MapFromMax, UniverseGenerationSettings.MapToMin, UniverseGenerationSettings.MapToMax);
            noiseValue = Mathf.Pow(noiseValue, UniverseGenerationSettings.Power);
            noiseValue *= UniverseGenerationSettings.Amplitude;
            #endregion

            return noiseValue;
        }
        #endregion

        #region Universe API

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

        public Filament[] GetLoadedFilaments()
        {
            return loadedFilaments.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsFilamentGenerated(Vector2Int filamentPosition)
        {
            string path = $"{DataPath}/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateFilament(Vector2Int filamentPosition)
        {
            if (IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is already generated!");
            }

            Filament filament = new Filament(filamentPosition);
            loadedFilaments.Add(filamentPosition, filament);
            SaveFilament(filament);
        }
        #endregion

        #region Saving
        public void SaveFilament(Filament filament)
        {
            string path = $"{DataPath}/Filaments/{filament.FilamentPosition.x}.{filament.FilamentPosition.y}.json";
            string json = JsonUtility.ToJson(filament, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveFilaments(Filament[] filaments)
        {
            foreach (Filament filament in filaments)
            {
                SaveFilament(filament);
            }
        }

        public void SaveLoadedFilaments()
        {
            SaveFilaments(loadedFilaments.Values.ToArray());
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

            string path = $"{DataPath}/Filaments/{filamentPosition.x}.{filamentPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Filament filament = JsonUtility.FromJson<Filament>(json);
            loadedFilaments.Add(filamentPosition, filament);
        }

        public void UnloadFilament(Vector2Int filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already unloaded!");
            }

            loadedFilaments.Remove(filamentPosition);
        }
        
        public void UnloadAllFilaments()
        {
            foreach (Vector2Int filamentPosition in loadedFilaments.Keys.ToArray())
            {
                UnloadFilament(filamentPosition);
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

        public Sector[] GetLoadedSectors()
        {
            return loadedSectors.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsSectorGenerated(Vector2Int sectorPosition)
        {
            string path = $"{DataPath}/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateSector(Vector2Int filamentPosition, Vector2Int sectorPosition)
        {
            if (!IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Containing Filament is not generated yet!");
            }
            if (IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Sector is already generated!");
            }

            Sector sector = new Sector(filamentPosition, sectorPosition);
            loadedSectors.Add(sectorPosition, sector);
            SaveSector(sector);
        }
        #endregion

        #region Saving
        public void SaveSector(Sector sector)
        {
            string path = $"{DataPath}/Sectors/{sector.SectorPosition.x}.{sector.SectorPosition.y}.json";
            string json = JsonUtility.ToJson(sector, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveSector(Sector[] sectors)
        {
            foreach (Sector sector in sectors)
            {
                SaveSector(sector);
            }
        }

        public void SaveLoadedSectors()
        {
            SaveSector(loadedSectors.Values.ToArray());
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

            string path = $"{DataPath}/Sectors/{sectorPosition.x}.{sectorPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Sector sector = JsonUtility.FromJson<Sector>(json);
            loadedSectors.Add(sectorPosition, sector);
        }

        public void UnloadSector(Vector2Int sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already unloaded!");
            }

            loadedSectors.Remove(sectorPosition);
        }
        
        public void UnloadAllSectors()
        {
            foreach (Vector2Int sectorPosition in loadedSectors.Keys.ToArray())
            {
                UnloadSector(sectorPosition);
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

        public Region[] GetLoadedRegions()
        {
            return loadedRegions.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsRegionGenerated(Vector2Int regionPosition)
        {
            string path = $"{DataPath}/Regions/{regionPosition.x}.{regionPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateRegion(Vector2Int sectorPosition, Vector2Int regionPosition)
        {
            if (!IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Containing Sector is not generated yet!");
            }
            if (IsRegionGenerated(regionPosition))
            {
                throw new Exception("Region is already generated!");
            }

            Region region = new Region(sectorPosition, regionPosition);
            loadedRegions.Add(regionPosition, region);
            SaveRegion(region);
        }
        #endregion

        #region Saving
        public void SaveRegion(Region region)
        {
            string path = $"{DataPath}/Regions/{region.RegionPosition.x}.{region.RegionPosition.y}.json";
            string json = JsonUtility.ToJson(region, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveRegions(Region[] regions)
        {
            foreach (Region region in regions)
            {
                SaveRegion(region);
            }
        }

        public void SaveRegions()
        {
            SaveRegions(loadedRegions.Values.ToArray());
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

            string path = $"{DataPath}/Regions/{regionPosition.x}.{regionPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Region region = JsonUtility.FromJson<Region>(json);
            loadedRegions.Add(regionPosition, region);
        }

        public void UnloadRegion(Vector2Int regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already unloaded!");
            }

            loadedRegions.Remove(regionPosition);
        }
        
        public void UnloadAllRegions()
        {
            foreach (Vector2Int regionPosition in loadedRegions.Keys.ToArray())
            {
                UnloadRegion(regionPosition);
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
                string path = $"{DataPath}/Regions/{regionPosition.x}.{regionPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #endregion
    }
}