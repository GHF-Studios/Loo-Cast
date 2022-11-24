using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;
using System.Linq;

namespace LooCast.Universe
{
    using Game;
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
                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public Vector2Int VectorIntPosition => vectorIntPosition;
                    public Filament.Position FilamentPosition => filamentPosition;
                    public Vector2 WorldPosition => worldPosition;
                    #endregion

                    #region Fields
                    [SerializeField] private Vector2Int vectorIntPosition;
                    [SerializeField] private Filament.Position filamentPosition;
                    [SerializeField] private Vector2 worldPosition;
                    #endregion

                    #region Constructors
                    public Position(Universe universe, Vector2Int vectorIntPosition, Filament.Position filamentPosition)
                    {
                        int chunkSize = universe.generationSettings.FilamentGenerationSettings.ChunkSize;
                        Vector2 chunkOffset = new Vector2(chunkSize / 2.0f, chunkSize / 2.0f);
                        this.vectorIntPosition = vectorIntPosition;
                        this.filamentPosition = filamentPosition;
                        worldPosition = vectorIntPosition / chunkSize + chunkOffset;
                    }

                    public Position(Universe universe, Vector2 worldPosition)
                    {
                        int chunkAmount = universe.generationSettings.FilamentGenerationSettings.ChunkAmount;
                        filamentPosition = new Filament.Position(universe, worldPosition);
                        vectorIntPosition = new Vector2Int(Mathf.RoundToInt(filamentPosition.VectorIntPosition.x / chunkAmount), Mathf.RoundToInt(filamentPosition.VectorIntPosition.y / chunkAmount));
                        this.worldPosition = worldPosition;
                    }
                    #endregion
                }
                #endregion

                #region Properties
                public int Size => size;
                public Position ChunkPosition => chunkPosition;
                public SerializableMap2D<float?> ElectronDensityMap => electronDensityMap;
                public SerializableMap2D<float?> PositronDensityMap => positronDensityMap;
                public SerializableMap2D<float?> ProtonDensityMap => protonDensityMap;
                public SerializableMap2D<float?> AntiProtonDensityMap => antiProtonDensityMap;
                public SerializableMap2D<float?> NeutronDensityMap => neutronDensityMap;
                public SerializableMap2D<float?> AntiNeutronDensityMap => antiNeutronDensityMap;
                #endregion

                #region Fields
                [SerializeField] private int size;
                [SerializeField] private Position chunkPosition;
                [SerializeField] private SerializableMap2D<float?> electronDensityMap;
                [SerializeField] private SerializableMap2D<float?> positronDensityMap;
                [SerializeField] private SerializableMap2D<float?> protonDensityMap;
                [SerializeField] private SerializableMap2D<float?> antiProtonDensityMap;
                [SerializeField] private SerializableMap2D<float?> neutronDensityMap;
                [SerializeField] private SerializableMap2D<float?> antiNeutronDensityMap;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Filament filament, Position chunkPosition)
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
                #endregion
            }
            #endregion

            #region Structs
            [Serializable]
            public struct Position
            {
                #region Properties
                public Vector2Int VectorIntPosition => vectorIntPosition;
                public Vector2 WorldPosition => worldPosition;
                #endregion

                #region Fields
                [SerializeField] private Vector2Int vectorIntPosition;
                [SerializeField] private Vector2 worldPosition;
                #endregion

                #region Constructor
                public Position(Universe universe, Vector2Int vectorIntPosition)
                {
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    int filamentSize = universe.generationSettings.FilamentGenerationSettings.Size;
                    Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                    Vector2 sectorOffset = new Vector2(sectorSize / 2.0f, sectorSize / 2.0f);
                    Vector2 filamentOffset = new Vector2(filamentSize / 2.0f, filamentSize / 2.0f);
                    this.vectorIntPosition = vectorIntPosition;
                    worldPosition = ((vectorIntPosition * regionSize + regionOffset) * sectorSize + sectorOffset) * filamentSize + filamentOffset;
                }

                public Position(Universe universe, Vector2 worldPosition)
                {
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    int filamentSize = universe.generationSettings.FilamentGenerationSettings.Size;
                    Vector2 floatFilamentPosition = worldPosition / regionSize / sectorSize / filamentSize;
                    vectorIntPosition = new Vector2Int(Mathf.RoundToInt(floatFilamentPosition.x), Mathf.RoundToInt(floatFilamentPosition.y));
                    this.worldPosition = worldPosition;
                }
                #endregion
            }

            [Serializable]
            public struct GenerationSettings
            {
                #region Properties
                public int Size
                {
                    get
                    {
                        return ChunkAmount * ChunkSize;
                    }
                }
                #endregion

                #region Fields
                [Header("Main Settings")]
                public GameObject Prefab;
                public int ChunkSize;
                public int ChunkAmount;
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
                #endregion
            }
            #endregion

            #region Properties
            public Position FilamentPosition => filamentPosition;
            public SerializableMap2D<Chunk> ChunkMap => chunkMap;
            #endregion

            #region Fields
            [SerializeField] private Position filamentPosition;
            [SerializeField] private SerializableMap2D<Chunk> chunkMap;
            #endregion

            #region Constructors
            public Filament(Position filamentPosition)
            {
                this.filamentPosition = filamentPosition;
            }
            #endregion

            #region Methods
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
            #endregion
        }

        [Serializable]
        public class Sector
        {
            #region Classes
            [Serializable]
            public class Chunk
            {
                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public Vector2Int VectorIntPosition => vectorIntPosition;
                    public Sector.Position SectorPosition => sectorPosition;
                    public Vector2 WorldPosition => worldPosition;
                    #endregion

                    #region Fields
                    [SerializeField] private Vector2Int vectorIntPosition;
                    [SerializeField] private Sector.Position sectorPosition;
                    [SerializeField] private Vector2 worldPosition;
                    #endregion

                    #region Constructors
                    public Position(Universe universe, Vector2Int vectorIntPosition, Sector.Position sectorPosition)
                    {
                        int chunkSize = universe.generationSettings.SectorGenerationSettings.ChunkSize;
                        Vector2 chunkOffset = new Vector2(chunkSize / 2.0f, chunkSize / 2.0f);
                        this.vectorIntPosition = vectorIntPosition;
                        this.sectorPosition = sectorPosition;
                        worldPosition = vectorIntPosition / chunkSize + chunkOffset;
                    }

                    public Position(Universe universe, Vector2 worldPosition)
                    {
                        int chunkAmount = universe.generationSettings.SectorGenerationSettings.ChunkAmount;
                        sectorPosition = new Sector.Position(universe, worldPosition);
                        vectorIntPosition = new Vector2Int(Mathf.RoundToInt(sectorPosition.VectorIntPosition.x / chunkAmount), Mathf.RoundToInt(sectorPosition.VectorIntPosition.y / chunkAmount));
                        this.worldPosition = worldPosition;
                    }
                    #endregion
                }
                #endregion

                #region Properties
                public int Size => size;
                public Position ChunkPosition => chunkPosition;
                public SerializableMap2D<float?> SolidParticleDensityMap => solidParticleDensityMap;
                public SerializableMap2D<float?> LiquidParticleDensityMap => liquidParticleDensityMap;
                public SerializableMap2D<float?> GasParticleDensityMap => gasParticleDensityMap;
                public SerializableMap2D<float?> PlasmaParticleDensityMap => plasmaParticleDensityMap;
                #endregion

                #region Fields
                [SerializeField] private int size;
                [SerializeField] private Position chunkPosition;
                [SerializeField] private SerializableMap2D<float?> solidParticleDensityMap;
                [SerializeField] private SerializableMap2D<float?> liquidParticleDensityMap;
                [SerializeField] private SerializableMap2D<float?> gasParticleDensityMap;
                [SerializeField] private SerializableMap2D<float?> plasmaParticleDensityMap;
                #endregion

                #region Methods
                public Chunk(Universe universe, Sector sector, Position chunkPosition)
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
                #endregion
            }
            #endregion

            #region Structs
            [Serializable]
            public struct Position
            {
                #region Properties
                public Vector2Int VectorIntPosition => vectorIntPosition;
                public Filament.Position FilamentPosition => filamentPosition;
                public Vector2 WorldPosition => worldPosition;
                #endregion

                #region Fields
                [SerializeField] private Vector2Int vectorIntPosition;
                [SerializeField] private Filament.Position filamentPosition;
                [SerializeField] private Vector2 worldPosition;
                #endregion

                #region Constructors
                public Position(Universe universe, Vector2Int vectorIntPosition, Filament.Position filamentPosition)
                {
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                    Vector2 sectorOffset = new Vector2(sectorSize / 2.0f, sectorSize / 2.0f);
                    this.vectorIntPosition = vectorIntPosition;
                    this.filamentPosition = filamentPosition;
                    worldPosition = (vectorIntPosition * regionSize + regionOffset) * sectorSize + sectorOffset;
                }

                public Position(Universe universe, Vector2 worldPosition)
                {
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    vectorIntPosition = Vector2Int.FloorToInt(worldPosition / regionSize / sectorSize);
                    filamentPosition = new Filament.Position(universe, worldPosition);
                    this.worldPosition = worldPosition;
                }
                #endregion
            }

            [Serializable]
            public struct GenerationSettings
            {
                #region Properties
                public int Size
                {
                    get
                    {
                        return ChunkAmount * ChunkSize;
                    }
                }
                #endregion

                #region Fields
                [Header("Main Settings")]
                public GameObject Prefab;
                public int ChunkSize;
                public int ChunkAmount;
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
                #endregion
            }
            #endregion

            #region Properties
            public Filament.Position FilamentPosition => filamentPosition;
            public Position SectorPosition => sectorPosition;
            public SerializableMap2D<Chunk> ChunkMap => chunkMap;
            #endregion

            #region Fields
            [SerializeField] private Filament.Position filamentPosition;
            [SerializeField] private Position sectorPosition;
            [SerializeField] private SerializableMap2D<Chunk> chunkMap;
            #endregion

            #region Methods
            public Sector(Filament.Position filamentPosition, Position sectorPosition)
            {
                this.filamentPosition = filamentPosition;
                this.sectorPosition = sectorPosition;
            }
            #endregion

            #region Methods
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
            #endregion
        }

        [Serializable]
        public class Region
        {
            #region Classes
            [Serializable]
            public class Chunk
            {
                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public Vector2Int VectorIntPosition => vectorIntPosition;
                    public Region.Position RegionPosition => regionPosition;
                    public Vector2 WorldPosition => worldPosition;
                    #endregion

                    #region Fields
                    [SerializeField] private Vector2Int vectorIntPosition;
                    [SerializeField] private Region.Position regionPosition;
                    [SerializeField] private Vector2 worldPosition;
                    #endregion

                    #region Constructors
                    public Position(Universe universe, Vector2Int vectorIntPosition, Region.Position regionPosition)
                    {
                        int chunkSize = universe.generationSettings.RegionGenerationSettings.ChunkSize;
                        Vector2 chunkOffset = new Vector2(chunkSize / 2.0f, chunkSize / 2.0f);
                        this.vectorIntPosition = vectorIntPosition;
                        this.regionPosition = regionPosition;
                        worldPosition = vectorIntPosition / chunkSize + chunkOffset;
                    }

                    public Position(Universe universe, Vector2 worldPosition)
                    {
                        int chunkSize = universe.generationSettings.RegionGenerationSettings.ChunkSize;
                        vectorIntPosition = Vector2Int.FloorToInt(new Region.Position(universe, worldPosition).VectorIntPosition * chunkSize);
                        regionPosition = new Region.Position(universe, worldPosition);
                        this.worldPosition = worldPosition;
                    }
                    #endregion
                }
                #endregion

                #region Properties
                public int Size => size;
                public Position ChunkPosition => chunkPosition;
                public SerializableMap2D<float?> MatterDensityMap => matterDensityMap;
                public SerializableMap2D<float?> AntiMatterDensityMap => antiMatterDensityMap;
                #endregion

                #region Fields
                [SerializeField] private int size;
                [SerializeField] private Position chunkPosition;
                [SerializeField] private SerializableMap2D<float?> matterDensityMap;
                [SerializeField] private SerializableMap2D<float?> antiMatterDensityMap;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Region region, Position chunkPosition)
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
                #endregion
            }
            #endregion

            #region Structs
            [Serializable]
            public struct Position
            {
                #region Properties
                public Vector2Int VectorIntPosition => vectorIntPosition;
                public Sector.Position SectorPosition => sectorPosition;
                public Vector2 WorldPosition => worldPosition;
                #endregion

                #region Fields
                [SerializeField] private Vector2Int vectorIntPosition;
                [SerializeField] private Sector.Position sectorPosition;
                [SerializeField] private Vector2 worldPosition;
                #endregion

                #region Constructors
                public Position(Universe universe, Vector2Int vectorIntPosition, Sector.Position sectorPosition)
                {
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                    this.vectorIntPosition = vectorIntPosition;
                    this.sectorPosition = sectorPosition;
                    worldPosition = vectorIntPosition * regionSize + regionOffset;
                }

                public Position(Universe universe, Vector2 worldPosition)
                {
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    vectorIntPosition = Vector2Int.FloorToInt(worldPosition / regionSize);
                    sectorPosition = new Sector.Position(universe, worldPosition);
                    this.worldPosition = worldPosition;
                }
                #endregion
            }

            [Serializable]
            public struct GenerationSettings
            {
                #region Properties
                public int Size
                {
                    get
                    {
                        return ChunkAmount * ChunkSize;
                    }
                }
                #endregion

                #region Fields
                [Header("Main Settings")]
                public GameObject Prefab;
                public int ChunkSize;
                public int ChunkAmount;
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
                #endregion
            }
            #endregion

            #region Properties
            public Sector.Position SectorPosition => sectorPosition;
            public Position RegionPosition => regionPosition;
            public SerializableMap2D<Chunk> ChunkMap => chunkMap;
            #endregion

            #region Fields
            [SerializeField] private Sector.Position sectorPosition;
            [SerializeField] private Position regionPosition;
            [SerializeField] private SerializableMap2D<Chunk> chunkMap;
            #endregion

            #region Constructors
            public Region(Sector.Position sectorPosition, Position regionPosition)
            {
                this.sectorPosition = sectorPosition;
                this.regionPosition = regionPosition;
            }
            #endregion

            #region Methods
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
            #endregion
        }
        #endregion

        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            #region Fields
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

            [Header("Sub Settings")]
            public Filament.GenerationSettings FilamentGenerationSettings;
            public Sector.GenerationSettings SectorGenerationSettings;
            public Region.GenerationSettings RegionGenerationSettings;
            #endregion
        }
        #endregion

        #region Static Properties
        public static string DataPath
        {
            get
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
        }
        public static GenerationSettings DefaultGenerationSettings
        {
            get
            {
                GenerationSettings generationSettings = new GenerationSettings();

                #region Universe Generation Settings Default
                generationSettings.Seed = 0;
                generationSettings.Size = 4096;
                generationSettings.MapFromMin = -1.0f;
                generationSettings.MapFromMax = 1.0f;
                generationSettings.MapToMin = 0.0f;
                generationSettings.MapToMax = 1.0f;
                generationSettings.Power = 5.0f;
                generationSettings.Amplitude = 64.0f;

                generationSettings.NoiseType = FastNoiseLite.NoiseType.Cellular;
                generationSettings.Frequency = 0.04f;

                generationSettings.FractalType = FastNoiseLite.FractalType.FBm;
                generationSettings.FractalOctaves = 3;
                generationSettings.FractalLacunarity = 2.0f;
                generationSettings.FractalGain = 0.5f;
                generationSettings.FractalWeightedStrength = 1.0f;

                generationSettings.CellularDistanceFunction = FastNoiseLite.CellularDistanceFunction.EuclideanSq;
                generationSettings.CellularReturnType = FastNoiseLite.CellularReturnType.Distance;
                generationSettings.CellularJitter = 1.0f;

                generationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
                generationSettings.DomainWarpAmplitude = 20.0f;
                generationSettings.DomainWarpFrequency = 0.01f;

                generationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
                generationSettings.DomainWarpFractalOctaves = 5;
                generationSettings.DomainWarpFractalLacunarity = 2.0f;
                generationSettings.DomainWarpFractalGain = 0.5f;
                #endregion

                #region Filament Generation Settings Default
                generationSettings.FilamentGenerationSettings.ChunkSize = 64;
                generationSettings.FilamentGenerationSettings.ChunkAmount = 64;
                generationSettings.FilamentGenerationSettings.MapFromMin = -1.0f;
                generationSettings.FilamentGenerationSettings.MapFromMax = 1.0f;
                generationSettings.FilamentGenerationSettings.MapToMin = 0.0f;
                generationSettings.FilamentGenerationSettings.MapToMax = 1.0f;
                generationSettings.FilamentGenerationSettings.UniverseNoiseInfluence = 1.0f;
                generationSettings.FilamentGenerationSettings.Power = 1.0f;
                generationSettings.FilamentGenerationSettings.Amplitude = 1.0f;

                generationSettings.FilamentGenerationSettings.NoiseType = FastNoiseLite.NoiseType.Cellular;
                generationSettings.FilamentGenerationSettings.Frequency = 0.02f;

                generationSettings.FilamentGenerationSettings.FractalType = FastNoiseLite.FractalType.FBm;
                generationSettings.FilamentGenerationSettings.FractalOctaves = 5;
                generationSettings.FilamentGenerationSettings.FractalLacunarity = 2.0f;
                generationSettings.FilamentGenerationSettings.FractalGain = 0.5f;
                generationSettings.FilamentGenerationSettings.FractalWeightedStrength = 0.0f;

                generationSettings.FilamentGenerationSettings.CellularDistanceFunction = FastNoiseLite.CellularDistanceFunction.EuclideanSq;
                generationSettings.FilamentGenerationSettings.CellularReturnType = FastNoiseLite.CellularReturnType.Distance;
                generationSettings.FilamentGenerationSettings.CellularJitter = 1.0f;

                generationSettings.FilamentGenerationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
                generationSettings.FilamentGenerationSettings.DomainWarpAmplitude = 20.0f;
                generationSettings.FilamentGenerationSettings.DomainWarpFrequency = 0.005f;

                generationSettings.FilamentGenerationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
                generationSettings.FilamentGenerationSettings.DomainWarpFractalOctaves = 5;
                generationSettings.FilamentGenerationSettings.DomainWarpFractalLacunarity = 2.0f;
                generationSettings.FilamentGenerationSettings.DomainWarpFractalGain = 0.5f;
                #endregion

                #region Sectror Generation Settings Default
                generationSettings.SectorGenerationSettings.ChunkSize = 64;
                generationSettings.SectorGenerationSettings.ChunkAmount = 64;
                generationSettings.SectorGenerationSettings.MapFromMin = -1.0f;
                generationSettings.SectorGenerationSettings.MapFromMax = 1.0f;
                generationSettings.SectorGenerationSettings.MapToMin = 0.0f;
                generationSettings.SectorGenerationSettings.MapToMax = 1.0f;
                generationSettings.SectorGenerationSettings.FilamentNoiseInfluence = 1.0f;
                generationSettings.SectorGenerationSettings.Power = 1.0f;
                generationSettings.SectorGenerationSettings.Amplitude = 1.0f;

                generationSettings.SectorGenerationSettings.NoiseType = FastNoiseLite.NoiseType.OpenSimplex2;
                generationSettings.SectorGenerationSettings.Frequency = 0.01f;

                generationSettings.SectorGenerationSettings.FractalType = FastNoiseLite.FractalType.FBm;
                generationSettings.SectorGenerationSettings.FractalOctaves = 5;
                generationSettings.SectorGenerationSettings.FractalLacunarity = 2.0f;
                generationSettings.SectorGenerationSettings.FractalGain = 0.5f;
                generationSettings.SectorGenerationSettings.FractalWeightedStrength = 0.0f;

                generationSettings.SectorGenerationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
                generationSettings.SectorGenerationSettings.DomainWarpAmplitude = 20.0f;
                generationSettings.SectorGenerationSettings.DomainWarpFrequency = 0.005f;

                generationSettings.SectorGenerationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
                generationSettings.SectorGenerationSettings.DomainWarpFractalOctaves = 5;
                generationSettings.SectorGenerationSettings.DomainWarpFractalLacunarity = 2.0f;
                generationSettings.SectorGenerationSettings.DomainWarpFractalGain = 0.5f;
                #endregion

                #region Region Generation Settings Default
                generationSettings.RegionGenerationSettings.ChunkSize = 64;
                generationSettings.RegionGenerationSettings.ChunkAmount = 64;
                generationSettings.RegionGenerationSettings.MapFromMin = -1.0f;
                generationSettings.RegionGenerationSettings.MapFromMax = 1.0f;
                generationSettings.RegionGenerationSettings.MapToMin = -0.375f;
                generationSettings.RegionGenerationSettings.MapToMax = 1.375f;
                generationSettings.RegionGenerationSettings.SectorNoiseInfluence = 1.0f;
                generationSettings.RegionGenerationSettings.Power = 1.0f;
                generationSettings.RegionGenerationSettings.Amplitude = 1.0f;

                generationSettings.RegionGenerationSettings.NoiseType = FastNoiseLite.NoiseType.OpenSimplex2;
                generationSettings.RegionGenerationSettings.Frequency = 0.005f;

                generationSettings.RegionGenerationSettings.FractalType = FastNoiseLite.FractalType.FBm;
                generationSettings.RegionGenerationSettings.FractalOctaves = 5;
                generationSettings.RegionGenerationSettings.FractalLacunarity = 2.0f;
                generationSettings.RegionGenerationSettings.FractalGain = 0.5f;
                generationSettings.RegionGenerationSettings.FractalWeightedStrength = 0.0f;

                generationSettings.RegionGenerationSettings.DomainWarpType = FastNoiseLite.DomainWarpType.OpenSimplex2;
                generationSettings.RegionGenerationSettings.DomainWarpAmplitude = 20.0f;
                generationSettings.RegionGenerationSettings.DomainWarpFrequency = 0.005f;

                generationSettings.RegionGenerationSettings.DomainWarpFractalType = FastNoiseLite.FractalType.DomainWarpProgressive;
                generationSettings.RegionGenerationSettings.DomainWarpFractalOctaves = 5;
                generationSettings.RegionGenerationSettings.DomainWarpFractalLacunarity = 2.0f;
                generationSettings.RegionGenerationSettings.DomainWarpFractalGain = 0.5f;
                #endregion

                return generationSettings;
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
                Directory.CreateDirectory(Path.GetDirectoryName(path));
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
        private Dictionary<Filament.Position, Filament> loadedFilaments;
        private Dictionary<Sector.Position, Sector> loadedSectors;
        private Dictionary<Region.Position, Region> loadedRegions;
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
        #endregion

        #region Methods
        public void Initialize()
        {
            loadedFilaments = new Dictionary<Filament.Position, Filament>();
            loadedSectors = new Dictionary<Sector.Position, Sector>();
            loadedRegions = new Dictionary<Region.Position, Region>();

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
        public Filament GetFilament(Filament.Position filamentPosition)
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
        public bool IsFilamentGenerated(Filament.Position filamentPosition)
        {
            string path = $"{DataPath}/Filaments/{filamentPosition.VectorIntPosition.x}.{filamentPosition.VectorIntPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateFilament(Filament.Position filamentPosition)
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
            string path = $"{DataPath}/Filaments/{filament.FilamentPosition.VectorIntPosition.x}.{filament.FilamentPosition.VectorIntPosition.y}.json";
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
        public bool IsFilamentLoaded(Filament.Position filamentPosition)
        {
            return loadedFilaments.ContainsKey(filamentPosition);
        }

        public void LoadFilament(Filament.Position filamentPosition)
        {
            if (IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already loaded!");
            }

            if (!IsFilamentGenerated(filamentPosition))
            {
                throw new Exception($"Filament has not been generated yet!");
            }

            string path = $"{DataPath}/Filaments/{filamentPosition.VectorIntPosition.x}.{filamentPosition.VectorIntPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Filament filament = JsonUtility.FromJson<Filament>(json);
            loadedFilaments.Add(filamentPosition, filament);
        }

        public void UnloadFilament(Filament.Position filamentPosition)
        {
            if (!IsFilamentLoaded(filamentPosition))
            {
                throw new Exception("Filament is already unloaded!");
            }

            loadedFilaments.Remove(filamentPosition);
        }
        
        public void UnloadAllFilaments()
        {
            foreach (Filament.Position filamentPosition in loadedFilaments.Keys.ToArray())
            {
                UnloadFilament(filamentPosition);
            }
        }
        #endregion

        #region Deletion
        public void DeleteFilament(Filament.Position filamentPosition)
        {
            if (IsFilamentLoaded(filamentPosition))
            {
                UnloadFilament(filamentPosition);
            }

            if (IsFilamentGenerated(filamentPosition))
            {
                string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.VectorIntPosition.x}.{filamentPosition.VectorIntPosition.y}.json";
                File.Delete(path);

                path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.VectorIntPosition.x}.{filamentPosition.VectorIntPosition.y}_Map.png";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #region Sectors

        #region Utility
        public Sector GetSector(Sector.Position sectorPosition)
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
        public bool IsSectorGenerated(Sector.Position sectorPosition)
        {
            string path = $"{DataPath}/Sectors/{sectorPosition.VectorIntPosition.x}.{sectorPosition.VectorIntPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateSector(Filament.Position filamentPosition, Sector.Position sectorPosition)
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
            string path = $"{DataPath}/Sectors/{sector.SectorPosition.VectorIntPosition.x}.{sector.SectorPosition.VectorIntPosition.y}.json";
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
        public bool IsSectorLoaded(Sector.Position sectorPosition)
        {
            return loadedSectors.ContainsKey(sectorPosition);
        }

        public void LoadSector(Sector.Position sectorPosition)
        {
            if (IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already loaded!");
            }

            if (!IsSectorGenerated(sectorPosition))
            {
                throw new Exception($"Sector has not been generated yet!");
            }

            string path = $"{DataPath}/Sectors/{sectorPosition.VectorIntPosition.x}.{sectorPosition.VectorIntPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Sector sector = JsonUtility.FromJson<Sector>(json);
            loadedSectors.Add(sectorPosition, sector);
        }

        public void UnloadSector(Sector.Position sectorPosition)
        {
            if (!IsSectorLoaded(sectorPosition))
            {
                throw new Exception("Sector is already unloaded!");
            }

            loadedSectors.Remove(sectorPosition);
        }
        
        public void UnloadAllSectors()
        {
            foreach (Sector.Position sectorPosition in loadedSectors.Keys.ToArray())
            {
                UnloadSector(sectorPosition);
            }
        }
        #endregion

        #region Deletion
        public void DeleteSector(Sector.Position sectorPosition)
        {
            if (IsSectorLoaded(sectorPosition))
            {
                UnloadSector(sectorPosition);
            }

            if (IsSectorGenerated(sectorPosition))
            {
                string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.VectorIntPosition.x}.{sectorPosition.VectorIntPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #region Regions

        #region Utility
        public Region GetRegion(Region.Position regionPosition)
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
        public bool IsRegionGenerated(Region.Position regionPosition)
        {
            string path = $"{DataPath}/Regions/{regionPosition.VectorIntPosition.x}.{regionPosition.VectorIntPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateRegion(Sector.Position sectorPosition, Region.Position regionPosition)
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
            string path = $"{DataPath}/Regions/{region.RegionPosition.VectorIntPosition.x}.{region.RegionPosition.VectorIntPosition.y}.json";
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
        public bool IsRegionLoaded(Region.Position regionPosition)
        {
            return loadedRegions.ContainsKey(regionPosition);
        }

        public void LoadRegion(Region.Position regionPosition)
        {
            if (IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already loaded!");
            }

            if (!IsRegionGenerated(regionPosition))
            {
                throw new Exception($"Region has not been generated yet!");
            }

            string path = $"{DataPath}/Regions/{regionPosition.VectorIntPosition.x}.{regionPosition.VectorIntPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Region region = JsonUtility.FromJson<Region>(json);
            loadedRegions.Add(regionPosition, region);
        }

        public void UnloadRegion(Region.Position regionPosition)
        {
            if (!IsRegionLoaded(regionPosition))
            {
                throw new Exception("Region is already unloaded!");
            }

            loadedRegions.Remove(regionPosition);
        }
        
        public void UnloadAllRegions()
        {
            foreach (Region.Position regionPosition in loadedRegions.Keys.ToArray())
            {
                UnloadRegion(regionPosition);
            }
        }
        #endregion

        #region Deletion
        public void DeleteRegion(Region.Position regionPosition)
        {
            if (IsRegionLoaded(regionPosition))
            {
                UnloadRegion(regionPosition);
            }

            if (IsRegionGenerated(regionPosition))
            {
                string path = $"{DataPath}/Regions/{regionPosition.VectorIntPosition.x}.{regionPosition.VectorIntPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #endregion
    }
}