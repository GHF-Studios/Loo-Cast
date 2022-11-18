using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;
using System.Linq;

namespace LooCast.Universe
{
    using Util;
    using Test;
    using Math.Map;
    using System.Collections.Generic;

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
                public Map2D<float?> ElectronDensityMap => electronDensityMap;
                public Map2D<float?> PositronDensityMap => positronDensityMap;
                public Map2D<float?> ProtonDensityMap => protonDensityMap;
                public Map2D<float?> AntiProtonDensityMap => antiProtonDensityMap;
                public Map2D<float?> NeutronDensityMap => neutronDensityMap;
                public Map2D<float?> AntiNeutronDensityMap => antiNeutronDensityMap;

                public Chunk(Filament filament, Vector2Int chunkPosition)
                {
                    GenerationSettings generationSettings = Instance.FilamentGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;

                    Vector2Int filamentPosition = filament.FilamentPosition;

                    #region Electron Density Map Generation
                    electronDensityMap = new Map2D<float?>(generationSettings.ChunkSize, generationSettings.ChunkSize);

                    for (int y = 0; y < generationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < generationSettings.ChunkSize; x++)
                        {
                            #region Filament Noise Sampling
                            float filamentOffsetX = -((filamentPosition.x * generationSettings.Size) + chunkPosition.x * generationSettings.ChunkSize);
                            float filamentOffsetY = -((filamentPosition.y * generationSettings.Size) + chunkPosition.y * generationSettings.ChunkSize);

                            float filamentSampleX = x + filamentOffsetX;
                            float filamentSampleY = y + filamentOffsetY;

                            float electronDensity = filament.SampleNoise(filamentSampleX, filamentSampleY);
                            // TODO: Sample all other density Maps, too
                            #endregion

                            #region Universe Noise Sampling
                            float universeOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                            float universeOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                            float universeSampleX = filamentPosition.x + universeOffsetX;
                            float universeSampleY = filamentPosition.y + universeOffsetY;

                            float universeNoiseValue = Instance.SampleNoise(universeSampleX, universeSampleY);
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
                [SerializeField] private Map2D<float?> electronDensityMap;
                [SerializeField] private Map2D<float?> positronDensityMap;
                [SerializeField] private Map2D<float?> protonDensityMap;
                [SerializeField] private Map2D<float?> antiProtonDensityMap;
                [SerializeField] private Map2D<float?> neutronDensityMap;
                [SerializeField] private Map2D<float?> antiNeutronDensityMap;
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
            public Map2D<Chunk> ChunkMap => chunkMap;

            [SerializeField] private Vector2Int filamentPosition;
            [SerializeField] private Map2D<Chunk> chunkMap;

            public Filament(Vector2Int filamentPosition)
            {
                this.filamentPosition = filamentPosition;
            }

            public float SampleNoise(float sampleX, float sampleY)
            {
                #region Sampling
                Instance.FilamentDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                float noiseValue = Instance.FilamentNoiseGenerator.GetNoise(sampleX, sampleY);
                #endregion

                #region Processing
                GenerationSettings generationSettings = Instance.FilamentGenerationSettings;
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

            [SerializeField] private Vector2Int filamentPosition;
            [SerializeField] private Vector2Int sectorPosition;

            private Texture2D map;

            public Sector(Vector2Int filamentPosition, Vector2Int sectorPosition)
            {
                GenerationSettings generationSettings = Instance.SectorGenerationSettings;
                this.filamentPosition = filamentPosition;
                this.sectorPosition = sectorPosition;

                #region Sector Map Generation
                Color[] noiseColorMap = new Color[generationSettings.Size * generationSettings.Size];
                Filament filament = Instance.GetFilament(filamentPosition);
                for (int y = 0; y < generationSettings.Size; y++)
                {
                    for (int x = 0; x < generationSettings.Size; x++)
                    {
                        #region Sector Noise Sampling
                        float sectorOffsetX = -(sectorPosition.x * generationSettings.Size);
                        float sectorOffsetY = -(sectorPosition.y * generationSettings.Size);

                        float sectorSampleX = x + sectorOffsetX;
                        float sectorSampleY = y + sectorOffsetY;

                        float sectorNoiseValue = SampleNoise(sectorSampleX, sectorSampleY);
                        #endregion

                        #region Filament Noise Sampling
                        float filamentOffsetX = -(1 / generationSettings.Size * x);
                        float filamentOffsetY = -(1 / generationSettings.Size * y);

                        float filamentSampleX = filamentPosition.x + filamentOffsetX;
                        float filamentSampleY = filamentPosition.y + filamentOffsetY;

                        float filamentNoiseValue = filament.SampleNoise(filamentSampleX, filamentSampleY);
                        #endregion

                        #region Total Noise Evaluation
                        //TODO: Change, Map is being called twice, in Filament and here, somehow fix this performance issue!
                        filamentNoiseValue = filamentNoiseValue.Map(0, 1, -1, 1);
                        float totalNoiseValue = sectorNoiseValue * (1 + (generationSettings.FilamentNoiseInfluence * filamentNoiseValue));
                        #endregion

                        noiseColorMap[y * generationSettings.Size + x] = new Color(totalNoiseValue, totalNoiseValue, totalNoiseValue, 1.0f);
                    }
                }

                map = TextureUtil.TextureFromColorMap(noiseColorMap, generationSettings.Size, generationSettings.Size);
                #endregion
            }

            public float SampleNoise(float sampleX, float sampleY)
            {
                #region Sampling
                Instance.SectorDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                float noiseValue = Instance.SectorNoiseGenerator.GetNoise(sampleX, sampleY);
                #endregion

                #region Processing
                GenerationSettings generationSettings = Instance.SectorGenerationSettings;
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

            public Vector2Int FilamentPosition => Instance.GetSector(sectorPosition).FilamentPosition;
            public Vector2Int SectorPosition => sectorPosition;
            public Vector2Int RegionPosition => regionPosition;
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

            [SerializeField] private Vector2Int sectorPosition;
            [SerializeField] private Vector2Int regionPosition;

            private Texture2D map;

            public Region(Vector2Int sectorPosition, Vector2Int regionPosition)
            {
                GenerationSettings generationSettings = Instance.RegionGenerationSettings;
                this.sectorPosition = sectorPosition;
                this.regionPosition = regionPosition;

                #region Region Map Generation
                Color[] noiseColorMap = new Color[generationSettings.Size * generationSettings.Size];
                Sector sector = Instance.GetSector(sectorPosition);
                for (int y = 0; y < generationSettings.Size; y++)
                {
                    for (int x = 0; x < generationSettings.Size; x++)
                    {
                        #region Region Noise Sampling
                        float regionOffsetX = -(regionPosition.x * generationSettings.Size);
                        float regionOffsetY = -(regionPosition.y * generationSettings.Size);

                        float regionSampleX = x + regionOffsetX;
                        float regionSampleY = y + regionOffsetY;

                        float regionNoiseValue = SampleNoise(regionSampleX, regionSampleY);
                        #endregion

                        #region Sector Noise Sampling
                        float sectorOffsetX = -(1 / generationSettings.Size * x);
                        float sectorOffsetY = -(1 / generationSettings.Size * y);

                        float sectorSampleX = sectorPosition.x + sectorOffsetX;
                        float sectorSampleY = sectorPosition.y + sectorOffsetY;

                        float sectorNoiseValue = sector.SampleNoise(sectorSampleX, sectorSampleY);
                        #endregion

                        #region Total Noise Evaluation
                        //TODO: Change, Map is being called twice, in Sector and here, somehow fix this performance issue!
                        sectorNoiseValue = sectorNoiseValue.Map(0, 1, -1, 1);
                        float totalNoiseValue = regionNoiseValue * (1 + (generationSettings.SectorNoiseInfluence * sectorNoiseValue));
                        #endregion

                        noiseColorMap[y * generationSettings.Size + x] = new Color(totalNoiseValue, totalNoiseValue, totalNoiseValue, 1.0f);
                    }
                }

                map = TextureUtil.TextureFromColorMap(noiseColorMap, generationSettings.Size, generationSettings.Size);
                #endregion
            }

            public float SampleNoise(float sampleX, float sampleY)
            {
                #region Sampling
                Instance.RegionDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                float noiseValue = Instance.RegionNoiseGenerator.GetNoise(sampleX, sampleY);
                #endregion

                #region Processing
                GenerationSettings generationSettings = Instance.RegionGenerationSettings;
                noiseValue = noiseValue.Map(generationSettings.MapFromMin, generationSettings.MapFromMax, generationSettings.MapToMin, generationSettings.MapToMax);
                noiseValue = Mathf.Pow(noiseValue, generationSettings.Power);
                noiseValue *= generationSettings.Amplitude;
                #endregion

                return noiseValue;
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
        #endregion

        #region Static Fields
        private static Universe instance;
        #endregion

        #region Properties
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

        private Dictionary<Vector2Int, Filament> loadedFilaments;
        private Dictionary<Vector2Int, Sector> loadedSectors;
        private Dictionary<Vector2Int, Region> loadedRegions;
        private Texture2D map;
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

            map = TextureUtil.TextureFromColorMap(noiseColorMap, UniverseGenerationSettings.Size, UniverseGenerationSettings.Size);
            #endregion
        }
        #endregion

        #region Methods
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

        #region Universe

        #region Initialization
        private void Initialize()
        {
            loadedFilaments = new Dictionary<Vector2Int, Filament>();
            loadedSectors = new Dictionary<Vector2Int, Sector>();
            loadedRegions = new Dictionary<Vector2Int, Region>();

            initialized = true;
        }
        #endregion

        #region Termination
        private void Terminate()
        {

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
            Instance.Map = new Texture2D(Instance.UniverseGenerationSettings.Size, Instance.UniverseGenerationSettings.Size);
            Instance.Map.filterMode = FilterMode.Point;
            Instance.Map.wrapMode = TextureWrapMode.Clamp;
            ImageConversion.LoadImage(Instance.Map, mapData);
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

        public Filament[] GetFilaments(Vector2Int[] filamentPositions)
        {
            Filament[] filaments = new Filament[filamentPositions.Length];
            for (int i = 0; i < filamentPositions.Length; i++)
            {
                filaments[i] = GetFilament(filamentPositions[i]);
            }
            return filaments;
        }

        public Filament[] GetFilaments()
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

            Filament filament = new Filament(filamentPosition);
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
        public void SaveFilament(Filament filament)
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

        public void SaveFilaments(Filament[] filaments)
        {
            foreach (Filament filament in filaments)
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

            Filament filament = GetFilament(filamentPosition);
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
            Filament filament = JsonUtility.FromJson<Filament>(json);
            loadedFilaments.Add(filamentPosition, filament);

            path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.x}.{filamentPosition.y}_Map.png";
            byte[] mapData = File.ReadAllBytes(path);
            filament.Map = new Texture2D(Instance.FilamentGenerationSettings.Size, Instance.FilamentGenerationSettings.Size);
            filament.Map.filterMode = FilterMode.Point;
            filament.Map.wrapMode = TextureWrapMode.Clamp;
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

        public Sector[] GetSectors(Vector2Int[] sectorPositions)
        {
            Sector[] sectors = new Sector[sectorPositions.Length];
            for (int i = 0; i < sectorPositions.Length; i++)
            {
                sectors[i] = GetSector(sectorPositions[i]);
            }
            return sectors;
        }

        public Sector[] GetSectors()
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

            Sector sector = new Sector(filamentPosition, sectorPosition);
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
        public void SaveSector(Sector sector)
        {
            string path = $"{Application.dataPath}/Data/Universe/Sectors/{sector.SectorPosition.x}.{sector.SectorPosition.y}.json";
            string json = JsonUtility.ToJson(sector, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);

            path = $"{Application.dataPath}/Data/Universe/Sectors/{sector.SectorPosition.x}.{sector.SectorPosition.y}_Map.png";
            byte[] mapData = ImageConversion.EncodeToPNG(sector.Map);
            using BinaryWriter binaryWriter = new BinaryWriter(File.OpenWrite(path));
            binaryWriter.Write(mapData);
        }

        public void SaveSector(Sector[] sectors)
        {
            foreach (Sector sector in sectors)
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

            Sector sector = GetSector(sectorPosition);
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
            Sector sector = JsonUtility.FromJson<Sector>(json);
            loadedSectors.Add(sectorPosition, sector);

            path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.x}.{sectorPosition.y}_Map.png";
            byte[] mapData = File.ReadAllBytes(path);
            sector.Map = new Texture2D(Instance.SectorGenerationSettings.Size, Instance.SectorGenerationSettings.Size);
            sector.Map.filterMode = FilterMode.Point;
            sector.Map.wrapMode = TextureWrapMode.Clamp;
            ImageConversion.LoadImage(sector.Map, mapData);
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

        public Region[] GetRegions(Vector2Int[] regionPositions)
        {
            Region[] regions = new Region[regionPositions.Length];
            for (int i = 0; i < regionPositions.Length; i++)
            {
                regions[i] = GetRegion(regionPositions[i]);
            }
            return regions;
        }

        public Region[] GetRegions()
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

            Region region = new Region(sectorPosition, regionPosition);
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
        public void SaveRegion(Region region)
        {
            string path = $"{Application.dataPath}/Data/Universe/Regions/{region.RegionPosition.x}.{region.RegionPosition.y}.json";
            string json = JsonUtility.ToJson(region, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);

            path = $"{Application.dataPath}/Data/Universe/Regions/{region.RegionPosition.x}.{region.RegionPosition.y}_Map.png";
            byte[] mapData = ImageConversion.EncodeToPNG(region.Map);
            using BinaryWriter binaryWriter = new BinaryWriter(File.OpenWrite(path));
            binaryWriter.Write(mapData);
        }

        public void SaveRegions(Region[] regions)
        {
            foreach (Region region in regions)
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

            Region region = GetRegion(regionPosition);
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
            Region region = JsonUtility.FromJson<Region>(json);
            loadedRegions.Add(regionPosition, region);

            path = $"{Application.dataPath}/Data/Universe/Sectors/{regionPosition.x}.{regionPosition.y}_Map.png";
            byte[] mapData = File.ReadAllBytes(path);
            region.Map = new Texture2D(Instance.RegionGenerationSettings.Size, Instance.RegionGenerationSettings.Size);
            region.Map.filterMode = FilterMode.Point;
            region.Map.wrapMode = TextureWrapMode.Clamp;
            ImageConversion.LoadImage(region.Map, mapData);
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

        #endregion
    }
}