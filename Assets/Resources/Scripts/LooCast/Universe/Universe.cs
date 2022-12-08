using UnityEngine;
using System.Collections.Generic;
using System.IO;
using System;
using System.Linq;
using System.Threading;
namespace LooCast.Universe
{
    using Core;
    using Game;
    using Util;
    using Util.Collections.Generic;
    using Random;
    using System.Collections.Concurrent;

    [Serializable]
    public class Universe
    {
        #region Classes
        [Serializable]
        public class Object : ExtendedMonoBehaviour
        {
            #region Classes
            [Serializable]
            public class Transform
            {
                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public double X
                    {
                        get
                        {
                            return x;
                        }

                        private set
                        {
                            x = value;
                        }
                    }
                    public double Y
                    {
                        get
                        {
                            return y;
                        }

                        private set
                        {
                            y = value;
                        }
                    }
                    public double Z
                    {
                        get
                        {
                            return z;
                        }

                        private set
                        {
                            z = value;
                        }
                    }
                    #endregion

                    #region Fields
                    [SerializeField] private double x;
                    [SerializeField] private double y;
                    [SerializeField] private double z;
                    #endregion

                    #region Constructors
                    public Position(double x, double y, double z)
                    {
                        this.x = x;
                        this.y = y;
                        this.z = z;
                    }
                    #endregion
                }
                #endregion

                #region Properties
                public Position UniversePosition
                {
                    get
                    {
                        return universePosition;
                    }

                    private set
                    {
                        universePosition = value;
                    }
                }
                #endregion

                #region Fields
                [SerializeField] private Position universePosition;
                #endregion

                #region Constructors
                private Transform()
                {

                }
                #endregion
            }
            #endregion
            
            #region Properties
            public Transform UniverseTransform
            {
                get
                {
                    {
                        return universeTransform;
                    }
                }

                private set
                {
                    universeTransform = value;
                }
            }
            #endregion

            #region Fields
            [SerializeField] private Transform universeTransform;
            #endregion
        }

        // TODO: Add a Universe.Transform to this class maybe?
        [Serializable]
        public class Filament
        {
            #region Classes
            [Serializable]
            public class Chunk
            {
                #region Enums
                public enum DensityMapType
                {
                    Electron,
                    Positron,
                    Proton,
                    AntiProton,
                    Neutron,
                    AntiNeutron
                }

                public enum GenerationState
                {
                    Generating,
                    Generated
                }
                #endregion

                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public Vector2Int ChunkPosition => chunkPosition;
                    public Vector2 WorldPosition
                    {
                        get
                        {
                            Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                            int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                            int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                            int filamentChunkSize = universe.generationSettings.FilamentGenerationSettings.ChunkSize;
                            Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                            return ((chunkPosition.ToVector2() * regionSize) + regionOffset) * sectorSize * filamentChunkSize;
                        }
                    }
                    public Region.Position RegionPosition
                    {
                        get
                        {
                            return new Region.Position(WorldPosition);
                        }
                    }
                    public Sector.Position SectorPosition
                    {
                        get
                        {
                            return new Sector.Position(WorldPosition);
                        }
                    }
                    public Filament.Position FilamentPosition
                    {
                        get
                        {
                            return new Filament.Position(WorldPosition);
                        }
                    }
                    public Region.Chunk.Position RegionChunkPosition
                    {
                        get
                        {
                            return new Region.Chunk.Position(WorldPosition);
                        }
                    }
                    public Sector.Chunk.Position SectorChunkPosition
                    {
                        get
                        {
                            return new Sector.Chunk.Position(WorldPosition);
                        }
                    }
                    #endregion

                    #region Fields
                    [SerializeField] private Vector2Int chunkPosition;
                    #endregion

                    #region Constructors
                    public Position(Vector2Int chunkPosition)
                    {
                        this.chunkPosition = chunkPosition;
                    }

                    public Position(Vector2 worldPosition)
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                        int filamentChunkSize = universe.generationSettings.FilamentGenerationSettings.ChunkSize;
                        chunkPosition = (worldPosition / regionSize / sectorSize / filamentChunkSize).FloorToVector2Int();
                    }
                    #endregion
                }

                [Serializable]
                public struct DensityMap
                {
                    public SerializableDictionary<Vector2Int, float> DensityMapDictionary => densityMapDictionary;
                    public DensityMapType DensityMapType => densityMapType;
                    
                    [SerializeField] private SerializableDictionary<Vector2Int, float> densityMapDictionary;
                    [SerializeField] private DensityMapType densityMapType;

                    public DensityMap(SerializableDictionary<Vector2Int, float> densityMapDictionary, DensityMapType densityMapType)
                    {
                        this.densityMapDictionary = densityMapDictionary;
                        this.densityMapType = densityMapType;
                    }
                }
                
                private struct DensityMapThreadInfo
                {
                    public readonly Action<DensityMapCollection> Callback;
                    public readonly DensityMapCollection DensityMaps;

                    public DensityMapThreadInfo(Action<DensityMapCollection> callback, DensityMapCollection densityMaps)
                    {
                        Callback = callback;
                        DensityMaps = densityMaps;
                    }
                }
                #endregion

                #region Classes
                [Serializable]
                public class DensityMapCollection
                {
                    [SerializeField] public DensityMap ElectronDensityMap;
                    [SerializeField] public DensityMap PositronDensityMap;
                    [SerializeField] public DensityMap ProtonDensityMap;
                    [SerializeField] public DensityMap AntiProtonDensityMap;
                    [SerializeField] public DensityMap NeutronDensityMap;
                    [SerializeField] public DensityMap AntiNeutronDensityMap;
                    [SerializeField] public GenerationState GenerationState;

                    public DensityMapCollection()
                    {
                        GenerationState = GenerationState.Generating;
                    }
                }
                #endregion

                #region Static Fields
                private static Queue<DensityMapThreadInfo> densityMapThreadInfoQueue = new Queue<DensityMapThreadInfo>();
                #endregion

                #region Properties
                public int ChunkSeed => chunkSeed;
                public int Size => size;
                public Position ChunkPosition => chunkPosition;
                public DensityMapCollection DensityMaps => densityMaps;
                #endregion

                #region Fields
                [SerializeField] private int chunkSeed;
                [SerializeField] private int size;
                [SerializeField] private Position chunkPosition;
                [SerializeField] private DensityMapCollection densityMaps;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Filament filament, Position chunkPosition)
                {
                    GenerationSettings generationSettings = universe.FilamentGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + filament.filamentPosition.ChunkPosition.magnitude + chunkPosition.ChunkPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection();
                    RequestDensityMaps(universe, filament, OnDensityMapsReceived);
                }
                #endregion

                #region Static Methods
                public static void ProcessDensityMapThreadInfoQueue()
                {
                    while (densityMapThreadInfoQueue.Count > 0)
                    {
                        DensityMapThreadInfo threadInfo = densityMapThreadInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);
                    }
                }
                #endregion

                #region Methods
                private DensityMap GenerateDensityMap(Universe universe, Filament filament, DensityMapType densityMapType)
                {
                    SerializableDictionary<Vector2Int, float> densityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                    GenerationSettings generationSettings = universe.FilamentGenerationSettings;

                    switch (densityMapType)
                    {
                        case DensityMapType.Electron:
                            for (int y = 0; y < generationSettings.ChunkSize; y++)
                            {
                                for (int x = 0; x < generationSettings.ChunkSize; x++)
                                {
                                    #region Filament Noise Sampling
                                    float filamentOffsetX = -((filament.FilamentPosition.ChunkPosition.x * generationSettings.Size) + chunkPosition.ChunkPosition.x * generationSettings.ChunkSize);
                                    float filamentOffsetY = -((filament.FilamentPosition.ChunkPosition.y * generationSettings.Size) + chunkPosition.ChunkPosition.y * generationSettings.ChunkSize);

                                    float filamentSampleX = x + filamentOffsetX;
                                    float filamentSampleY = y + filamentOffsetY;

                                    float electronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                                    // TODO: Sample all other density Maps, too
                                    #endregion

                                    #region Universe Noise Sampling
                                    float universeOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                                    float universeOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                                    float universeSampleX = filament.FilamentPosition.ChunkPosition.x + universeOffsetX;
                                    float universeSampleY = filament.FilamentPosition.ChunkPosition.y + universeOffsetY;

                                    float universeNoiseValue = universe.SampleNoise(universeSampleX, universeSampleY);
                                    // TODO: Sample all other density Maps, too
                                    #endregion

                                    #region Total Density Evaluation
                                    universeNoiseValue = universeNoiseValue.Map(0, 1, -1, 1);
                                    float totalElectronDensity = electronDensity * (1 + (generationSettings.UniverseNoiseInfluence * universeNoiseValue));
                                    #endregion

                                    densityMapDictionary.Add(new Vector2Int(x, y), totalElectronDensity);
                                }
                            }
                            break;
                        case DensityMapType.Positron:
                            break;
                        case DensityMapType.Proton:
                            break;
                        case DensityMapType.AntiProton:
                            break;
                        case DensityMapType.Neutron:
                            break;
                        case DensityMapType.AntiNeutron:
                            break;
                        default:
                            break;
                    }

                    return new DensityMap(densityMapDictionary, densityMapType);
                }
                
                private void RequestDensityMaps(Universe universe, Filament filament, Action<DensityMapCollection> callback)
                {
                    ThreadStart threadStart = delegate
                    {
                        DensityMapGenerationThread(universe, filament, callback);
                    };

                    new Thread(threadStart).Start();
                }

                private void OnDensityMapsReceived(DensityMapCollection densityMaps)
                {
                    this.densityMaps.ElectronDensityMap = densityMaps.ElectronDensityMap;
                    this.densityMaps.PositronDensityMap = densityMaps.PositronDensityMap;
                    this.densityMaps.ProtonDensityMap = densityMaps.ProtonDensityMap;
                    this.densityMaps.AntiProtonDensityMap = densityMaps.AntiProtonDensityMap;
                    this.densityMaps.NeutronDensityMap = densityMaps.NeutronDensityMap;
                    this.densityMaps.AntiNeutronDensityMap = densityMaps.AntiNeutronDensityMap;
                    this.densityMaps.GenerationState = densityMaps.GenerationState;
                    GameManager.Instance.CurrentGame.CurrentUniverse.SaveFilamentChunk(this);
                }

                private void DensityMapGenerationThread(Universe universe, Filament filament, Action<DensityMapCollection> callback)
                {
                    DensityMapCollection densityMaps = new DensityMapCollection();
                    densityMaps.ElectronDensityMap = GenerateDensityMap(universe, filament, DensityMapType.Electron);
                    densityMaps.PositronDensityMap = GenerateDensityMap(universe, filament, DensityMapType.Positron);
                    densityMaps.ProtonDensityMap = GenerateDensityMap(universe, filament, DensityMapType.Proton);
                    densityMaps.AntiProtonDensityMap = GenerateDensityMap(universe, filament, DensityMapType.AntiProton);
                    densityMaps.NeutronDensityMap = GenerateDensityMap(universe, filament, DensityMapType.Neutron);
                    densityMaps.AntiNeutronDensityMap = GenerateDensityMap(universe, filament, DensityMapType.AntiNeutron);
                    densityMaps.GenerationState = GenerationState.Generated;
                    
                    lock (densityMapThreadInfoQueue)
                    {
                        densityMapThreadInfoQueue.Enqueue(new DensityMapThreadInfo(callback, densityMaps));
                    }
                }
                #endregion
            }
            #endregion

            #region Structs
            [Serializable]
            public struct Position
            {
                #region Properties
                public Vector2Int ChunkPosition => chunkPosition;
                public Vector2 WorldPosition
                {
                    get
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                        int filamentSize = universe.generationSettings.FilamentGenerationSettings.Size;
                        Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                        return ((chunkPosition * regionSize) + regionOffset) * sectorSize * filamentSize;
                    }
                }
                public Region.Position RegionPosition
                {
                    get
                    {
                        return new Region.Position(WorldPosition);
                    }
                }
                public Sector.Position SectorPosition
                {
                    get
                    {
                        return new Sector.Position(WorldPosition);
                    }
                }
                public Region.Chunk.Position RegionChunkPosition
                {
                    get
                    {
                        return new Region.Chunk.Position(WorldPosition);
                    }
                }
                public Sector.Chunk.Position SectorChunkPosition
                {
                    get
                    {
                        return new Sector.Chunk.Position(WorldPosition);
                    }
                }
                public Filament.Chunk.Position FilamentChunkPosition
                {
                    get
                    {
                        return new Filament.Chunk.Position(WorldPosition);
                    }
                }
                #endregion

                #region Fields
                [SerializeField] private Vector2Int chunkPosition;
                #endregion

                #region Constructor
                public Position(Vector2Int chunkPosition)
                {
                    this.chunkPosition = chunkPosition;
                }

                public Position(Vector2 worldPosition)
                {
                    Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    int filamentSize = universe.generationSettings.FilamentGenerationSettings.Size;
                    chunkPosition = (worldPosition / regionSize / sectorSize / filamentSize).FloorToVector2Int();
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
            public SerializableDictionary<Vector2Int, Chunk.Position> ChunkPositionMap => chunkPositionMap;
            #endregion

            #region Fields
            [SerializeField] private Position filamentPosition;
            [SerializeField] private SerializableDictionary<Vector2Int, Chunk.Position> chunkPositionMap;
            #endregion

            #region Constructors
            public Filament(Universe universe, Position filamentPosition)
            {
                this.filamentPosition = filamentPosition;
                chunkPositionMap = new SerializableDictionary<Vector2Int, Chunk.Position>();
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

            public void RegisterChunkPosition(Chunk.Position chunkPosition)
            {
                if (chunkPositionMap.ContainsKey(chunkPosition.ChunkPosition))
                {
                    throw new Exception("Chunk.Position is already registered!");
                }
                chunkPositionMap.Add(chunkPosition.ChunkPosition, chunkPosition);
            }

            public void UnregisterChunkPosition(Chunk.Position chunkPosition)
            {
                chunkPositionMap.Remove(chunkPosition.ChunkPosition);
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
                #region Enums
                public enum DensityMapType
                {
                    SolidParticle,
                    LiquidParticle,
                    GasParticle,
                    PlasmaParticle
                }

                public enum GenerationState
                {
                    Generating,
                    Generated
                }
                #endregion

                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public Vector2Int ChunkPosition => chunkPosition;
                    public Vector2 WorldPosition
                    {
                        get
                        {
                            Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                            int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                            int sectorChunkSize = universe.generationSettings.SectorGenerationSettings.ChunkSize;
                            Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                            return ((chunkPosition * regionSize) + regionOffset) * sectorChunkSize;
                        }
                    }
                    public Region.Position RegionPosition
                    {
                        get
                        {
                            return new Region.Position(WorldPosition);
                        }
                    }
                    public Sector.Position SectorPosition
                    {
                        get
                        {
                            return new Sector.Position(WorldPosition);
                        }
                    }
                    public Filament.Position FilamentPosition
                    {
                        get
                        {
                            return new Filament.Position(WorldPosition);
                        }
                    }
                    public Region.Chunk.Position RegionChunkPosition
                    {
                        get
                        {
                            return new Region.Chunk.Position(WorldPosition);
                        }
                    }
                    public Filament.Chunk.Position FilamentChunkPosition
                    {
                        get
                        {
                            return new Filament.Chunk.Position(WorldPosition);
                        }
                    }
                    #endregion

                    #region Fields
                    [SerializeField] private Vector2Int chunkPosition;
                    #endregion

                    #region Constructors
                    public Position(Vector2Int chunkPosition)
                    {
                        this.chunkPosition = chunkPosition;
                    }

                    public Position(Vector2 worldPosition)
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorChunkSize = universe.generationSettings.SectorGenerationSettings.ChunkSize;
                        chunkPosition = (worldPosition / regionSize / sectorChunkSize).FloorToVector2Int();
                    }
                    #endregion
                }

                [Serializable]
                public struct DensityMap
                {
                    public SerializableDictionary<Vector2Int, float> DensityMapDictionary => densityMapDictionary;
                    public DensityMapType DensityMapType => densityMapType;

                    [SerializeField] private SerializableDictionary<Vector2Int, float> densityMapDictionary;
                    [SerializeField] private DensityMapType densityMapType;

                    public DensityMap(SerializableDictionary<Vector2Int, float> densityMapDictionary, DensityMapType densityMapType)
                    {
                        this.densityMapDictionary = densityMapDictionary;
                        this.densityMapType = densityMapType;
                    }
                }

                private struct DensityMapThreadInfo
                {
                    public readonly Action<DensityMapCollection> Callback;
                    public readonly DensityMapCollection DensityMaps;

                    public DensityMapThreadInfo(Action<DensityMapCollection> callback, DensityMapCollection densityMaps)
                    {
                        Callback = callback;
                        DensityMaps = densityMaps;
                    }
                }
                #endregion

                #region Classes
                [Serializable]
                public class DensityMapCollection
                {
                    [SerializeField] public DensityMap SolidParticleDensityMap;
                    [SerializeField] public DensityMap LiquidParticleDensityMap;
                    [SerializeField] public DensityMap GasParticleDensityMap;
                    [SerializeField] public DensityMap PlasmaParticleDensityMap;
                    [SerializeField] public GenerationState GenerationState;

                    public DensityMapCollection()
                    {
                        GenerationState = GenerationState.Generating;
                    }
                }
                #endregion

                #region Static Fields
                private static Queue<DensityMapThreadInfo> densityMapThreadInfoQueue = new Queue<DensityMapThreadInfo>();
                #endregion

                #region Properties
                public int ChunkSeed => chunkSeed;
                public int Size => size;
                public Position ChunkPosition => chunkPosition;
                public DensityMapCollection DensityMaps => densityMaps;
                #endregion

                #region Fields
                [SerializeField] private int chunkSeed;
                [SerializeField] private int size;
                [SerializeField] private Position chunkPosition;
                [SerializeField] private DensityMapCollection densityMaps;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Sector sector, Position chunkPosition)
                {
                    GenerationSettings generationSettings = universe.SectorGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + sector.sectorPosition.ChunkPosition.magnitude + chunkPosition.ChunkPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection();
                    RequestDensityMaps(universe, sector, OnDensityMapsReceived);
                }
                #endregion

                #region Static Methods
                public static void ProcessDensityMapThreadInfoQueue()
                {
                    while (densityMapThreadInfoQueue.Count > 0)
                    {
                        DensityMapThreadInfo threadInfo = densityMapThreadInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);
                    }
                }
                #endregion
                
                #region Methods
                private DensityMap GenerateDensityMap(Universe universe, Sector sector, DensityMapType densityMapType)
                {
                    SerializableDictionary<Vector2Int, float> densityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                    GenerationSettings generationSettings = universe.SectorGenerationSettings;

                    switch (densityMapType)
                    {
                        case DensityMapType.SolidParticle:
                            for (int y = 0; y < generationSettings.ChunkSize; y++)
                            {
                                for (int x = 0; x < generationSettings.ChunkSize; x++)
                                {
                                    #region Sector Noise Sampling
                                    float sectorOffsetX = -((sector.SectorPosition.ChunkPosition.x * generationSettings.Size) + chunkPosition.ChunkPosition.x * generationSettings.ChunkSize);
                                    float sectorOffsetY = -((sector.SectorPosition.ChunkPosition.y * generationSettings.Size) + chunkPosition.ChunkPosition.y * generationSettings.ChunkSize);

                                    float sectorSampleX = x + sectorOffsetX;
                                    float sectorSampleY = y + sectorOffsetY;

                                    float solidParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                                    // TODO: Sample all other density Maps, too
                                    #endregion

                                    #region Sector Noise Sampling
                                    float filamentOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                                    float filamentOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                                    float filamentSampleX = sector.SectorPosition.ChunkPosition.x + filamentOffsetX;
                                    float filamentSampleY = sector.SectorPosition.ChunkPosition.y + filamentOffsetY;

                                    float filamentNoiseValue = universe.SampleNoise(filamentSampleX, filamentSampleY);
                                    // TODO: Sample all other density Maps, too
                                    #endregion

                                    #region Total Density Evaluation
                                    filamentNoiseValue = filamentNoiseValue.Map(0, 1, -1, 1);
                                    float totalSolidParticleDensity = solidParticleDensity * (1 + (generationSettings.FilamentNoiseInfluence * filamentNoiseValue));
                                    #endregion

                                    densityMapDictionary.Add(new Vector2Int(x, y), totalSolidParticleDensity);
                                }
                            }
                            break;
                        case DensityMapType.LiquidParticle:
                            break;
                        case DensityMapType.GasParticle:
                            break;
                        case DensityMapType.PlasmaParticle:
                            break;
                        default:
                            break;
                    }

                    return new DensityMap(densityMapDictionary, densityMapType);
                }

                private void RequestDensityMaps(Universe universe, Sector sector, Action<DensityMapCollection> callback)
                {
                    ThreadStart threadStart = delegate
                    {
                        DensityMapGenerationThread(universe, sector, callback);
                    };

                    new Thread(threadStart).Start();
                }

                private void OnDensityMapsReceived(DensityMapCollection densityMaps)
                {
                    this.densityMaps.SolidParticleDensityMap = densityMaps.SolidParticleDensityMap;
                    this.densityMaps.LiquidParticleDensityMap = densityMaps.LiquidParticleDensityMap;
                    this.densityMaps.GasParticleDensityMap = densityMaps.GasParticleDensityMap;
                    this.densityMaps.PlasmaParticleDensityMap = densityMaps.PlasmaParticleDensityMap;
                    this.densityMaps.GenerationState = densityMaps.GenerationState;
                    GameManager.Instance.CurrentGame.CurrentUniverse.SaveSectorChunk(this);
                }

                private void DensityMapGenerationThread(Universe universe, Sector sector, Action<DensityMapCollection> callback)
                {
                    DensityMapCollection densityMaps = new DensityMapCollection();
                    densityMaps.SolidParticleDensityMap = GenerateDensityMap(universe, sector, DensityMapType.SolidParticle);
                    densityMaps.LiquidParticleDensityMap = GenerateDensityMap(universe, sector, DensityMapType.LiquidParticle);
                    densityMaps.GasParticleDensityMap = GenerateDensityMap(universe, sector, DensityMapType.GasParticle);
                    densityMaps.PlasmaParticleDensityMap = GenerateDensityMap(universe, sector, DensityMapType.PlasmaParticle);
                    densityMaps.GenerationState = GenerationState.Generated;

                    lock (densityMapThreadInfoQueue)
                    {
                        densityMapThreadInfoQueue.Enqueue(new DensityMapThreadInfo(callback, densityMaps));
                    }
                }
                #endregion
            }
            #endregion

            #region Structs
            [Serializable]
            public struct Position
            {
                #region Properties
                public Vector2Int ChunkPosition => chunkPosition;
                public Vector2 WorldPosition
                {
                    get
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                        Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                        return ((chunkPosition * regionSize) + regionOffset) * sectorSize;
                    }
                }
                public Region.Position RegionPosition
                {
                    get
                    {
                        return new Region.Position(WorldPosition);
                    }
                }
                public Filament.Position FilamentPosition
                {
                    get
                    {
                        return new Filament.Position(WorldPosition);
                    }
                }
                public Region.Chunk.Position RegionChunkPosition
                {
                    get
                    {
                        return new Region.Chunk.Position(WorldPosition);
                    }
                }
                public Sector.Chunk.Position SectorChunkPosition
                {
                    get
                    {
                        return new Sector.Chunk.Position(WorldPosition);
                    }
                }
                public Filament.Chunk.Position FilamentChunkPosition
                {
                    get
                    {
                        return new Filament.Chunk.Position(WorldPosition);
                    }
                }
                #endregion

                #region Fields
                [SerializeField] private Vector2Int chunkPosition;
                #endregion

                #region Constructors
                public Position(Vector2Int chunkPosition)
                {
                    this.chunkPosition = chunkPosition;
                }

                public Position(Vector2 worldPosition)
                {
                    Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    chunkPosition = (worldPosition / regionSize / sectorSize).FloorToVector2Int();
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
            public Position SectorPosition => sectorPosition;
            public SerializableDictionary<Vector2Int, Chunk.Position> ChunkPositionMap => chunkPositionMap;
            #endregion

            #region Fields
            [SerializeField] private Position sectorPosition;
            [SerializeField] private SerializableDictionary<Vector2Int, Chunk.Position> chunkPositionMap;
            #endregion

            #region Constructors
            public Sector(Universe universe, Position sectorPosition)
            {
                this.sectorPosition = sectorPosition;
                chunkPositionMap = new SerializableDictionary<Vector2Int, Chunk.Position>();
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

            public void RegisterChunkPosition(Chunk.Position chunkPosition)
            {
                if (chunkPositionMap.ContainsKey(chunkPosition.ChunkPosition))
                {
                    throw new Exception("Chunk.Position is already registered!");
                }
                chunkPositionMap.Add(chunkPosition.ChunkPosition, chunkPosition);
            }

            public void UnregisterChunkPosition(Chunk.Position chunkPosition)
            {
                chunkPositionMap.Remove(chunkPosition.ChunkPosition);
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
                #region Enums
                public enum DensityMapType
                {
                    Matter,
                    AntiMatter
                }

                public enum GenerationState
                {
                    Generating,
                    Generated
                }
                #endregion

                #region Structs
                [Serializable]
                public struct Position
                {
                    #region Properties
                    public Vector2Int ChunkPosition => chunkPosition;
                    public Vector2 WorldPosition
                    {
                        get
                        {
                            Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                            int regionChunkSize = universe.generationSettings.RegionGenerationSettings.ChunkSize;
                            Vector2 regionChunkOffset = new Vector2(regionChunkSize / 2.0f, regionChunkSize / 2.0f);
                            return (chunkPosition.ToVector2() * regionChunkSize) + regionChunkOffset;
                        }
                    }
                    public Region.Position RegionPosition
                    {
                        get
                        {
                            return new Region.Position(WorldPosition);
                        }
                    }
                    public Sector.Position SectorPosition
                    {
                        get
                        {
                            return new Sector.Position(WorldPosition);
                        }
                    }
                    public Filament.Position FilamentPosition
                    {
                        get
                        {
                            return new Filament.Position(WorldPosition);
                        }
                    }
                    public Sector.Chunk.Position SectorChunkPosition
                    {
                        get
                        {
                            return new Sector.Chunk.Position(WorldPosition);
                        }
                    }
                    public Filament.Chunk.Position FilamentChunkPosition
                    {
                        get
                        {
                            return new Filament.Chunk.Position(WorldPosition);
                        }
                    }
                    #endregion

                    #region Fields
                    [SerializeField] private Vector2Int chunkPosition;
                    #endregion

                    #region Constructors
                    public Position(Vector2Int chunkPosition)
                    {
                        this.chunkPosition = chunkPosition;
                    }

                    public Position(Vector2 worldPosition)
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionChunkSize = universe.generationSettings.RegionGenerationSettings.ChunkSize;
                        chunkPosition = (worldPosition / regionChunkSize).FloorToVector2Int();
                    }
                    #endregion
                }

                [Serializable]
                public struct DensityMap
                {
                    public SerializableDictionary<Vector2Int, float> DensityMapDictionary => densityMapDictionary;
                    public DensityMapType DensityMapType => densityMapType;

                    [SerializeField] private SerializableDictionary<Vector2Int, float> densityMapDictionary;
                    [SerializeField] private DensityMapType densityMapType;

                    public DensityMap(SerializableDictionary<Vector2Int, float> densityMapDictionary, DensityMapType densityMapType)
                    {
                        this.densityMapDictionary = densityMapDictionary;
                        this.densityMapType = densityMapType;
                    }
                }

                private struct DensityMapThreadInfo
                {
                    public readonly Action<DensityMapCollection> Callback;
                    public readonly DensityMapCollection DensityMaps;

                    public DensityMapThreadInfo(Action<DensityMapCollection> callback, DensityMapCollection densityMaps)
                    {
                        Callback = callback;
                        DensityMaps = densityMaps;
                    }
                }
                #endregion

                #region Classes
                [Serializable]
                public class DensityMapCollection
                {
                    [SerializeField] public DensityMap MatterDensityMap;
                    [SerializeField] public DensityMap AntiMatterDensityMap;
                    [SerializeField] public GenerationState GenerationState;

                    public DensityMapCollection()
                    {
                        GenerationState = GenerationState.Generating;
                    }
                }
                #endregion

                #region Static Fields
                private static Queue<DensityMapThreadInfo> densityMapThreadInfoQueue = new Queue<DensityMapThreadInfo>();
                #endregion

                #region Properties
                public int ChunkSeed => chunkSeed;
                public int Size => size;
                public Position ChunkPosition => chunkPosition;
                public DensityMapCollection DensityMaps => densityMaps;
                #endregion

                #region Fields
                [SerializeField] private int chunkSeed;
                [SerializeField] private int size;
                [SerializeField] private Position chunkPosition;
                [SerializeField] private DensityMapCollection densityMaps;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Region region, Position chunkPosition)
                {
                    GenerationSettings generationSettings = universe.RegionGenerationSettings;
                    size = generationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + region.regionPosition.ChunkPosition.magnitude + chunkPosition.ChunkPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection();
                    RequestDensityMaps(universe, region, OnDensityMapsReceived);
                }
                #endregion

                #region Static Methods
                public static void ProcessDensityMapThreadInfoQueue()
                {
                    while (densityMapThreadInfoQueue.Count > 0)
                    {
                        DensityMapThreadInfo threadInfo = densityMapThreadInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);
                    }
                }
                #endregion

                #region Methods
                private DensityMap GenerateDensityMap(Universe universe, Region region, DensityMapType densityMapType)
                {
                    SerializableDictionary<Vector2Int, float> densityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                    GenerationSettings generationSettings = universe.RegionGenerationSettings;

                    switch (densityMapType)
                    {
                        case DensityMapType.Matter:
                            for (int y = 0; y < generationSettings.ChunkSize; y++)
                            {
                                for (int x = 0; x < generationSettings.ChunkSize; x++)
                                {
                                    #region Region Noise Sampling
                                    float regionOffsetX = -((region.RegionPosition.ChunkPosition.x * generationSettings.Size) + chunkPosition.ChunkPosition.x * generationSettings.ChunkSize);
                                    float regionOffsetY = -((region.RegionPosition.ChunkPosition.y * generationSettings.Size) + chunkPosition.ChunkPosition.y * generationSettings.ChunkSize);

                                    float regionSampleX = x + regionOffsetX;
                                    float regionSampleY = y + regionOffsetY;

                                    float matterDensity = region.SampleNoise(universe, regionSampleX, regionSampleY);
                                    // TODO: Sample all other density Maps, too
                                    #endregion

                                    #region Sector Noise Sampling
                                    float sectorOffsetX = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * x);
                                    float sectorOffsetY = -(1 / generationSettings.ChunkAmount / generationSettings.ChunkSize * y);

                                    float sectorSampleX = region.RegionPosition.ChunkPosition.x + sectorOffsetX;
                                    float sectorSampleY = region.RegionPosition.ChunkPosition.y + sectorOffsetY;

                                    float sectorNoiseValue = universe.SampleNoise(sectorSampleX, sectorSampleY);
                                    // TODO: Sample all other density Maps, too
                                    #endregion

                                    #region Total Density Evaluation
                                    sectorNoiseValue = sectorNoiseValue.Map(0, 1, -1, 1);
                                    float totalMatterDensity = matterDensity * (1 + (generationSettings.SectorNoiseInfluence * sectorNoiseValue));
                                    #endregion

                                    densityMapDictionary.Add(new Vector2Int(x, y), totalMatterDensity);
                                }
                            }
                            break;
                        case DensityMapType.AntiMatter:
                            break;
                        default:
                            break;
                    }

                    return new DensityMap(densityMapDictionary, densityMapType);
                }

                private void RequestDensityMaps(Universe universe, Region region, Action<DensityMapCollection> callback)
                {
                    ThreadStart threadStart = delegate
                    {
                        DensityMapGenerationThread(universe, region, callback);
                    };

                    new Thread(threadStart).Start();
                }

                private void OnDensityMapsReceived(DensityMapCollection densityMaps)
                {
                    this.densityMaps.MatterDensityMap = densityMaps.MatterDensityMap;
                    this.densityMaps.AntiMatterDensityMap = densityMaps.AntiMatterDensityMap;
                    this.densityMaps.GenerationState = densityMaps.GenerationState;
                    GameManager.Instance.CurrentGame.CurrentUniverse.SaveRegionChunk(this);
                }

                private void DensityMapGenerationThread(Universe universe, Region region, Action<DensityMapCollection> callback)
                {
                    DensityMapCollection densityMaps = new DensityMapCollection();
                    densityMaps.MatterDensityMap = GenerateDensityMap(universe, region, DensityMapType.Matter);
                    densityMaps.AntiMatterDensityMap = GenerateDensityMap(universe, region, DensityMapType.AntiMatter);
                    densityMaps.GenerationState = GenerationState.Generated;

                    lock (densityMapThreadInfoQueue)
                    {
                        densityMapThreadInfoQueue.Enqueue(new DensityMapThreadInfo(callback, densityMaps));
                    }
                }
                #endregion
            }
            #endregion

            #region Structs
            [Serializable]
            public struct Position
            {
                #region Properties
                public Vector2Int ChunkPosition => chunkPosition;
                public Vector2 WorldPosition
                {
                    get
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                        return (chunkPosition * regionSize) + regionOffset;
                    }
                }
                public Sector.Position SectorPosition
                {
                    get
                    {
                        return new Sector.Position(WorldPosition);
                    }
                }
                public Filament.Position FilamentPosition
                {
                    get
                    {
                        return new Filament.Position(WorldPosition);
                    }
                }
                public Region.Chunk.Position RegionChunkPosition
                {
                    get
                    {
                        return new Region.Chunk.Position(WorldPosition);
                    }
                }
                public Sector.Chunk.Position SectorChunkPosition
                {
                    get
                    {
                        return new Sector.Chunk.Position(WorldPosition);
                    }
                }
                public Filament.Chunk.Position FilamentChunkPosition
                {
                    get
                    {
                        return new Filament.Chunk.Position(WorldPosition);
                    }
                }
                #endregion

                #region Fields
                [SerializeField] private Vector2Int chunkPosition;
                #endregion

                #region Constructors
                public Position(Vector2Int chunkPosition)
                {
                    this.chunkPosition = chunkPosition;
                }

                public Position(Vector2 worldPosition)
                {
                    Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    chunkPosition = (worldPosition / regionSize).FloorToVector2Int();
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
            public Position RegionPosition => regionPosition;
            public SerializableDictionary<Vector2Int, Chunk.Position> ChunkPositionMap => chunkPositionMap;
            #endregion

            #region Fields
            [SerializeField] private Position regionPosition;
            [SerializeField] private SerializableDictionary<Vector2Int, Chunk.Position> chunkPositionMap;
            #endregion

            #region Constructors
            public Region(Universe universe, Position regionPosition)
            {
                this.regionPosition = regionPosition;
                chunkPositionMap = new SerializableDictionary<Vector2Int, Chunk.Position>();
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

            public void RegisterChunkPosition(Chunk.Position chunkPosition)
            {
                if (chunkPositionMap.ContainsKey(chunkPosition.ChunkPosition))
                {
                    throw new Exception("Chunk.Position is already registered!");
                }
                chunkPositionMap.Add(chunkPosition.ChunkPosition, chunkPosition);
            }

            public void UnregisterChunkPosition(Chunk.Position chunkPosition)
            {
                chunkPositionMap.Remove(chunkPosition.ChunkPosition);
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
                generationSettings.Size = 1024;
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
                generationSettings.FilamentGenerationSettings.ChunkSize = 16;
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
                generationSettings.SectorGenerationSettings.ChunkSize = 16;
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
                generationSettings.RegionGenerationSettings.ChunkSize = 16;
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
        public HashSet<Filament> LoadedFilaments
        {
            get
            {
                return loadedFilaments.Values.ToHashSet();
            }
        }
        public HashSet<Sector> LoadedSectors
        {
            get
            {
                return loadedSectors.Values.ToHashSet();
            }
        }
        public HashSet<Region> LoadedRegions
        {
            get
            {
                return loadedRegions.Values.ToHashSet();
            }
        }
        public HashSet<Filament.Chunk> LoadedFilamentChunks
        {
            get
            {
                return loadedFilamentChunks.Values.ToHashSet();
            }
        }
        public HashSet<Sector.Chunk> LoadedSectorChunks
        {
            get
            {
                return loadedSectorChunks.Values.ToHashSet();
            }
        }
        public HashSet<Region.Chunk> LoadedRegionChunks
        {
            get
            {
                return loadedRegionChunks.Values.ToHashSet();
            }
        }
        public HashSet<Filament.Position> LoadedFilamentPositions
        {
            get
            {
                return loadedFilaments.Keys.ToHashSet();
            }
        }
        public HashSet<Sector.Position> LoadedSectorPositions
        {
            get
            {
                return loadedSectors.Keys.ToHashSet();
            }
        }
        public HashSet<Region.Position> LoadedRegionPositions
        {
            get
            {
                return loadedRegions.Keys.ToHashSet();
            }
        }
        public HashSet<Filament.Chunk.Position> LoadedFilamentChunkPositions
        {
            get
            {
                return loadedFilamentChunks.Keys.ToHashSet();
            }
        }
        public HashSet<Sector.Chunk.Position> LoadedSectorChunkPositions
        {
            get
            {
                return loadedSectorChunks.Keys.ToHashSet();
            }
        }
        public HashSet<Region.Chunk.Position> LoadedRegionChunkPositions
        {
            get
            {
                return loadedRegionChunks.Keys.ToHashSet();
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
        private Dictionary<Filament.Chunk.Position, Filament.Chunk> loadedFilamentChunks;
        private Dictionary<Sector.Chunk.Position, Sector.Chunk> loadedSectorChunks;
        private Dictionary<Region.Chunk.Position, Region.Chunk> loadedRegionChunks;
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
            loadedFilamentChunks = new Dictionary<Filament.Chunk.Position, Filament.Chunk>();
            loadedSectorChunks = new Dictionary<Sector.Chunk.Position, Sector.Chunk>();
            loadedRegionChunks = new Dictionary<Region.Chunk.Position, Region.Chunk>();

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
            string path = $"{DataPath}/Filaments/{filamentPosition.ChunkPosition.x}.{filamentPosition.ChunkPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateFilament(Filament.Position filamentPosition)
        {
            if (IsFilamentGenerated(filamentPosition))
            {
                throw new Exception("Filament is already generated!");
            }

            Filament filament = new Filament(this, filamentPosition);
            loadedFilaments.Add(filamentPosition, filament);
            SaveFilament(filament);
        }
        #endregion

        #region Saving
        public void SaveFilament(Filament filament)
        {
            string path = $"{DataPath}/Filaments/{filament.FilamentPosition.ChunkPosition.x}.{filament.FilamentPosition.ChunkPosition.y}.json";
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

            string path = $"{DataPath}/Filaments/{filamentPosition.ChunkPosition.x}.{filamentPosition.ChunkPosition.y}.json";
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
                string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentPosition.ChunkPosition.x}.{filamentPosition.ChunkPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #region Filament Chunks

        #region Utility
        public Filament.Chunk GetFilamentChunk(Filament.Chunk.Position filamentChunkPosition)
        {
            if (!IsFilamentChunkLoaded(filamentChunkPosition))
            {
                throw new Exception("Filament.Chunk is not loaded!");
            }

            if (!IsFilamentChunkGenerated(filamentChunkPosition))
            {
                throw new Exception("Filament.Chunk is not generated!");
            }

            return loadedFilamentChunks[filamentChunkPosition];
        }

        public Filament.Chunk[] GetLoadedFilamentChunks()
        {
            return loadedFilamentChunks.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsFilamentChunkGenerated(Filament.Chunk.Position filamentChunkPosition)
        {
            string path = $"{DataPath}/Filaments/{filamentChunkPosition.FilamentPosition.ChunkPosition.x}.{filamentChunkPosition.FilamentPosition.ChunkPosition.y}/Chunks/{filamentChunkPosition.ChunkPosition.x}.{filamentChunkPosition.ChunkPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateFilamentChunk(Filament.Chunk.Position filamentChunkPosition)
        {
            if (IsFilamentChunkGenerated(filamentChunkPosition))
            {
                throw new Exception("Filament.Chunk is already generated!");
            }

            if (!IsFilamentGenerated(filamentChunkPosition.FilamentPosition))
            {
                throw new Exception("Containing Filament is not yet generated!");
            }

            if (!IsFilamentLoaded(filamentChunkPosition.FilamentPosition))
            {
                throw new Exception("Containing Filament is not yet loaded!");
            }

            Filament filament = GetFilament(filamentChunkPosition.FilamentPosition);
            Filament.Chunk filamentChunk = new Filament.Chunk(this, filament, filamentChunkPosition);
            filament.RegisterChunkPosition(filamentChunkPosition);
            loadedFilamentChunks.Add(filamentChunkPosition, filamentChunk);
            SaveFilament(filament);
            SaveFilamentChunk(filamentChunk);
        }
        #endregion

        #region Saving
        public void SaveFilamentChunk(Filament.Chunk filamentChunk)
        {
            string path = $"{DataPath}/Filaments/{filamentChunk.ChunkPosition.FilamentPosition.ChunkPosition.x}.{filamentChunk.ChunkPosition.FilamentPosition.ChunkPosition.y}/Chunks/{filamentChunk.ChunkPosition.ChunkPosition.x}.{filamentChunk.ChunkPosition.ChunkPosition.y}.json";
            string json = JsonUtility.ToJson(filamentChunk, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveFilamentChunks(Filament.Chunk[] filamentChunks)
        {
            foreach (Filament.Chunk filamentChunk in filamentChunks)
            {
                SaveFilamentChunk(filamentChunk);
            }
        }

        public void SaveFilamentChunks()
        {
            SaveFilamentChunks(loadedFilamentChunks.Values.ToArray());
        }
        #endregion

        #region Loading
        public bool IsFilamentChunkLoaded(Filament.Chunk.Position filamentChunkPosition)
        {
            return loadedFilamentChunks.ContainsKey(filamentChunkPosition);
        }

        public void LoadFilamentChunk(Filament.Chunk.Position filamentChunkPosition)
        {
            if (IsFilamentChunkLoaded(filamentChunkPosition))
            {
                throw new Exception("Filament.Chunk is already loaded!");
            }

            if (!IsFilamentChunkGenerated(filamentChunkPosition))
            {
                throw new Exception($"Filament.Chunk has not been generated yet!");
            }

            if (!IsFilamentGenerated(filamentChunkPosition.FilamentPosition))
            {
                throw new Exception("Containing Filament is not yet generated!");
            }

            if (!IsFilamentLoaded(filamentChunkPosition.FilamentPosition))
            {
                throw new Exception("Containing Filament is not yet loaded!");
            }

            string path = $"{DataPath}/Filaments/{filamentChunkPosition.FilamentPosition.ChunkPosition.x}.{filamentChunkPosition.FilamentPosition.ChunkPosition.y}/Chunks/{filamentChunkPosition.ChunkPosition.x}.{filamentChunkPosition.ChunkPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Filament.Chunk filamentChunk = JsonUtility.FromJson<Filament.Chunk>(json);
            loadedFilamentChunks.Add(filamentChunkPosition, filamentChunk);
        }

        public void UnloadFilamentChunk(Filament.Chunk.Position filamentChunkPosition)
        {
            if (!IsFilamentChunkLoaded(filamentChunkPosition))
            {
                throw new Exception("Filament.Chunk is already unloaded!");
            }

            loadedFilamentChunks.Remove(filamentChunkPosition);
        }

        public void UnloadAllFilamentChunks()
        {
            foreach (Filament.Chunk.Position filamentChunkPosition in loadedFilamentChunks.Keys.ToArray())
            {
                UnloadFilamentChunk(filamentChunkPosition);
            }
        }
        #endregion

        #region Deletion
        public void DeleteFilamentChunk(Filament.Chunk.Position filamentChunkPosition)
        {
            if (IsFilamentChunkLoaded(filamentChunkPosition))
            {
                UnloadFilamentChunk(filamentChunkPosition);
            }

            if (IsFilamentChunkGenerated(filamentChunkPosition))
            {
                Filament filament = GetFilament(filamentChunkPosition.FilamentPosition);
                filament.UnregisterChunkPosition(filamentChunkPosition);
                SaveFilament(filament);

                string path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentChunkPosition.ChunkPosition.x}.{filamentChunkPosition.ChunkPosition.y}/Chunks/{filamentChunkPosition.ChunkPosition.x}.{filamentChunkPosition.ChunkPosition.y}.json";
                File.Delete(path);

                path = $"{Application.dataPath}/Data/Universe/Filaments/{filamentChunkPosition.ChunkPosition.x}.{filamentChunkPosition.ChunkPosition.y}/Chunks/{filamentChunkPosition.ChunkPosition.x}.{filamentChunkPosition.ChunkPosition.y}_Map.png";
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
            string path = $"{DataPath}/Sectors/{sectorPosition.ChunkPosition.x}.{sectorPosition.ChunkPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateSector(Sector.Position sectorPosition)
        {
            if (!IsFilamentGenerated(sectorPosition.FilamentPosition))
            {
                throw new Exception("Containing Filament is not generated yet!");
            }
            if (IsSectorGenerated(sectorPosition))
            {
                throw new Exception("Sector is already generated!");
            }

            Sector sector = new Sector(this, sectorPosition);
            loadedSectors.Add(sectorPosition, sector);
            SaveSector(sector);
        }
        #endregion

        #region Saving
        public void SaveSector(Sector sector)
        {
            string path = $"{DataPath}/Sectors/{sector.SectorPosition.ChunkPosition.x}.{sector.SectorPosition.ChunkPosition.y}.json";
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

            string path = $"{DataPath}/Sectors/{sectorPosition.ChunkPosition.x}.{sectorPosition.ChunkPosition.y}.json";
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
                string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorPosition.ChunkPosition.x}.{sectorPosition.ChunkPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #region Sector Chunks

        #region Utility
        public Sector.Chunk GetSectorChunk(Sector.Chunk.Position sectorChunkPosition)
        {
            if (!IsSectorChunkLoaded(sectorChunkPosition))
            {
                throw new Exception("Sector.Chunk is not loaded!");
            }

            if (!IsSectorChunkGenerated(sectorChunkPosition))
            {
                throw new Exception("Sector.Chunk is not generated!");
            }

            return loadedSectorChunks[sectorChunkPosition];
        }

        public Sector.Chunk[] GetLoadedSectorChunks()
        {
            return loadedSectorChunks.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsSectorChunkGenerated(Sector.Chunk.Position sectorChunkPosition)
        {
            string path = $"{DataPath}/Sectors/{sectorChunkPosition.SectorPosition.ChunkPosition.x}.{sectorChunkPosition.SectorPosition.ChunkPosition.y}/Chunks/{sectorChunkPosition.ChunkPosition.x}.{sectorChunkPosition.ChunkPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateSectorChunk(Sector.Chunk.Position sectorChunkPosition)
        {
            if (IsSectorChunkGenerated(sectorChunkPosition))
            {
                throw new Exception("Sector Chunk is already generated!");
            }

            if (!IsSectorGenerated(sectorChunkPosition.SectorPosition))
            {
                throw new Exception("Containing Sector is not yet generated!");
            }

            if (!IsSectorLoaded(sectorChunkPosition.SectorPosition))
            {
                throw new Exception("Containing Sector is not yet loaded!");
            }

            Sector sector = GetSector(sectorChunkPosition.SectorPosition);
            Sector.Chunk sectorChunk = new Sector.Chunk(this, sector, sectorChunkPosition);
            sector.RegisterChunkPosition(sectorChunkPosition);
            loadedSectorChunks.Add(sectorChunkPosition, sectorChunk);
            SaveSector(sector);
            SaveSectorChunk(sectorChunk);
        }
        #endregion

        #region Saving
        public void SaveSectorChunk(Sector.Chunk sectorChunk)
        {
            string path = $"{DataPath}/Sectors/{sectorChunk.ChunkPosition.SectorPosition.ChunkPosition.x}.{sectorChunk.ChunkPosition.SectorPosition.ChunkPosition.y}/Chunks/{sectorChunk.ChunkPosition.ChunkPosition.x}.{sectorChunk.ChunkPosition.ChunkPosition.y}.json";
            string json = JsonUtility.ToJson(sectorChunk, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveSectorChunks(Sector.Chunk[] sectorChunks)
        {
            foreach (Sector.Chunk sectorChunk in sectorChunks)
            {
                SaveSectorChunk(sectorChunk);
            }
        }

        public void SaveSectorChunks()
        {
            SaveSectorChunks(loadedSectorChunks.Values.ToArray());
        }
        #endregion

        #region Loading
        public bool IsSectorChunkLoaded(Sector.Chunk.Position sectorChunkPosition)
        {
            return loadedSectorChunks.ContainsKey(sectorChunkPosition);
        }

        public void LoadSectorChunk(Sector.Chunk.Position sectorChunkPosition)
        {
            if (IsSectorChunkLoaded(sectorChunkPosition))
            {
                throw new Exception("Sector.Chunk is already loaded!");
            }

            if (!IsSectorChunkGenerated(sectorChunkPosition))
            {
                throw new Exception($"Sector.Chunk has not been generated yet!");
            }

            if (!IsSectorGenerated(sectorChunkPosition.SectorPosition))
            {
                throw new Exception("Containing Sector is not yet generated!");
            }

            if (!IsSectorLoaded(sectorChunkPosition.SectorPosition))
            {
                throw new Exception("Containing Sector is not yet loaded!");
            }

            string path = $"{DataPath}/Sectors/{sectorChunkPosition.SectorPosition.ChunkPosition.x}.{sectorChunkPosition.SectorPosition.ChunkPosition.y}/Chunks/{sectorChunkPosition.ChunkPosition.x}.{sectorChunkPosition.ChunkPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Sector.Chunk sectorChunk = JsonUtility.FromJson<Sector.Chunk>(json);
            loadedSectorChunks.Add(sectorChunkPosition, sectorChunk);
        }

        public void UnloadSectorChunk(Sector.Chunk.Position sectorChunkPosition)
        {
            if (!IsSectorChunkLoaded(sectorChunkPosition))
            {
                throw new Exception("Sector.Chunk is already unloaded!");
            }

            loadedSectorChunks.Remove(sectorChunkPosition);
        }

        public void UnloadAllSectorChunks()
        {
            foreach (Sector.Chunk.Position sectorChunkPosition in loadedSectorChunks.Keys.ToArray())
            {
                UnloadSectorChunk(sectorChunkPosition);
            }
        }
        #endregion

        #region Deletion
        public void DeleteSectorChunk(Sector.Chunk.Position sectorChunkPosition)
        {
            if (IsSectorChunkLoaded(sectorChunkPosition))
            {
                UnloadSectorChunk(sectorChunkPosition);
            }

            if (IsSectorChunkGenerated(sectorChunkPosition))
            {
                Sector sector = GetSector(sectorChunkPosition.SectorPosition);
                sector.UnregisterChunkPosition(sectorChunkPosition);
                SaveSector(sector);

                string path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorChunkPosition.ChunkPosition.x}.{sectorChunkPosition.ChunkPosition.y}/Chunks/{sectorChunkPosition.ChunkPosition.x}.{sectorChunkPosition.ChunkPosition.y}.json";
                File.Delete(path);

                path = $"{Application.dataPath}/Data/Universe/Sectors/{sectorChunkPosition.ChunkPosition.x}.{sectorChunkPosition.ChunkPosition.y}/Chunks/{sectorChunkPosition.ChunkPosition.x}.{sectorChunkPosition.ChunkPosition.y}_Map.png";
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
            string path = $"{DataPath}/Regions/{regionPosition.ChunkPosition.x}.{regionPosition.ChunkPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateRegion(Region.Position regionPosition)
        {
            if (!IsSectorGenerated(regionPosition.SectorPosition))
            {
                throw new Exception("Containing Sector is not generated yet!");
            }
            if (IsRegionGenerated(regionPosition))
            {
                throw new Exception("Region is already generated!");
            }

            Region region = new Region(this, regionPosition);
            loadedRegions.Add(regionPosition, region);
            SaveRegion(region);
        }
        #endregion

        #region Saving
        public void SaveRegion(Region region)
        {
            string path = $"{DataPath}/Regions/{region.RegionPosition.ChunkPosition.x}.{region.RegionPosition.ChunkPosition.y}.json";
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

            string path = $"{DataPath}/Regions/{regionPosition.ChunkPosition.x}.{regionPosition.ChunkPosition.y}.json";
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
                string path = $"{DataPath}/Regions/{regionPosition.ChunkPosition.x}.{regionPosition.ChunkPosition.y}.json";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #region Region Chunks

        #region Utility
        public Region.Chunk GetRegionChunk(Region.Chunk.Position regionChunkPosition)
        {
            if (!IsRegionChunkLoaded(regionChunkPosition))
            {
                throw new Exception("Region.Chunk is not loaded!");
            }

            if (!IsRegionChunkGenerated(regionChunkPosition))
            {
                throw new Exception("Region.Chunk is not generated!");
            }

            return loadedRegionChunks[regionChunkPosition];
        }

        public Region.Chunk[] GetLoadedRegionChunks()
        {
            return loadedRegionChunks.Values.ToArray();
        }
        #endregion

        #region Generation
        public bool IsRegionChunkGenerated(Region.Chunk.Position regionChunkPosition)
        {
            string path = $"{DataPath}/Regions/{regionChunkPosition.RegionPosition.ChunkPosition.x}.{regionChunkPosition.RegionPosition.ChunkPosition.y}/Chunks/{regionChunkPosition.ChunkPosition.x}.{regionChunkPosition.ChunkPosition.y}.json";
            return File.Exists(path);
        }

        public void GenerateRegionChunk(Region.Chunk.Position regionChunkPosition)
        {
            if (IsRegionChunkGenerated(regionChunkPosition))
            {
                throw new Exception("Region Chunk is already generated!");
            }

            if (!IsRegionGenerated(regionChunkPosition.RegionPosition))
            {
                throw new Exception("Containing Region is not yet generated!");
            }

            if (!IsRegionLoaded(regionChunkPosition.RegionPosition))
            {
                throw new Exception("Containing Region is not yet loaded!");
            }

            Region region = GetRegion(regionChunkPosition.RegionPosition);
            Region.Chunk regionChunk = new Region.Chunk(this, region, regionChunkPosition);
            region.RegisterChunkPosition(regionChunkPosition);
            loadedRegionChunks.Add(regionChunkPosition, regionChunk);
            SaveRegion(region);
            SaveRegionChunk(regionChunk);
        }
        #endregion

        #region Saving
        public void SaveRegionChunk(Region.Chunk regionChunk)
        {
            string path = $"{DataPath}/Regions/{regionChunk.ChunkPosition.RegionPosition.ChunkPosition.x}.{regionChunk.ChunkPosition.RegionPosition.ChunkPosition.y}/Chunks/{regionChunk.ChunkPosition.ChunkPosition.x}.{regionChunk.ChunkPosition.ChunkPosition.y}.json";
            string json = JsonUtility.ToJson(regionChunk, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter writer = new StreamWriter(path);
            writer.Write(json);
        }

        public void SaveRegionChunks(Region.Chunk[] regionChunks)
        {
            foreach (Region.Chunk regionChunk in regionChunks)
            {
                SaveRegionChunk(regionChunk);
            }
        }

        public void SaveRegionChunks()
        {
            SaveRegionChunks(loadedRegionChunks.Values.ToArray());
        }
        #endregion

        #region Loading
        public bool IsRegionChunkLoaded(Region.Chunk.Position regionChunkPosition)
        {
            return loadedRegionChunks.ContainsKey(regionChunkPosition);
        }

        public void LoadRegionChunk(Region.Chunk.Position regionChunkPosition)
        {
            if (IsRegionChunkLoaded(regionChunkPosition))
            {
                throw new Exception("Region.Chunk is already loaded!");
            }

            if (!IsRegionChunkGenerated(regionChunkPosition))
            {
                throw new Exception($"Region.Chunk has not been generated yet!");
            }

            if (!IsRegionGenerated(regionChunkPosition.RegionPosition))
            {
                throw new Exception("Containing Region is not yet generated!");
            }

            if (!IsRegionLoaded(regionChunkPosition.RegionPosition))
            {
                throw new Exception("Containing Region is not yet loaded!");
            }

            string path = $"{DataPath}/Regions/{regionChunkPosition.RegionPosition.ChunkPosition.x}.{regionChunkPosition.RegionPosition.ChunkPosition.y}/Chunks/{regionChunkPosition.ChunkPosition.x}.{regionChunkPosition.ChunkPosition.y}.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Region.Chunk regionChunk = JsonUtility.FromJson<Region.Chunk>(json);
            loadedRegionChunks.Add(regionChunkPosition, regionChunk);
        }

        public void UnloadRegionChunk(Region.Chunk.Position regionChunkPosition)
        {
            if (!IsRegionChunkLoaded(regionChunkPosition))
            {
                throw new Exception("Region.Chunk is already unloaded!");
            }

            loadedRegionChunks.Remove(regionChunkPosition);
        }

        public void UnloadAllRegionChunks()
        {
            foreach (Region.Chunk.Position regionChunkPosition in loadedRegionChunks.Keys.ToArray())
            {
                UnloadRegionChunk(regionChunkPosition);
            }
        }
        #endregion

        #region Deletion
        public void DeleteRegionChunk(Region.Chunk.Position regionChunkPosition)
        {
            if (IsRegionChunkLoaded(regionChunkPosition))
            {
                UnloadRegionChunk(regionChunkPosition);
            }

            if (IsRegionChunkGenerated(regionChunkPosition))
            {
                Region region = GetRegion(regionChunkPosition.RegionPosition);
                region.UnregisterChunkPosition(regionChunkPosition);
                SaveRegion(region);

                string path = $"{Application.dataPath}/Data/Universe/Regions/{regionChunkPosition.ChunkPosition.x}.{regionChunkPosition.ChunkPosition.y}/Chunks/{regionChunkPosition.ChunkPosition.x}.{regionChunkPosition.ChunkPosition.y}.json";
                File.Delete(path);

                path = $"{Application.dataPath}/Data/Universe/Regions/{regionChunkPosition.ChunkPosition.x}.{regionChunkPosition.ChunkPosition.y}/Chunks/{regionChunkPosition.ChunkPosition.x}.{regionChunkPosition.ChunkPosition.y}_Map.png";
                File.Delete(path);
            }
        }
        #endregion

        #endregion

        #endregion
    }
}