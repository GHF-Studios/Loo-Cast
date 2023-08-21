﻿using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
<<<<<<< HEAD:Assets/Resources/Scripts/LooCast/Universe/Universe.cs
    using LooCast.System.ECS;
    using LooCast.System.Lua;
=======
    using Core;
    using Game;
    using Random;
    using Util;
    using Util.Collections.Generic;
>>>>>>> develop:Assets/Mods/LooCast/Modules/Universe/Assets/Scripts/Universe.cs

    [LuaNamespace("LooCast.Universe")]
    public sealed class Universe : Entity
    {
<<<<<<< HEAD:Assets/Resources/Scripts/LooCast/Universe/Universe.cs
=======
        #region Classes
        public class DensityMapGenerationUtil : MonoBehaviour
        {
            #region Structs
            private struct FilamentDensityMapCoroutineInfo
            {
                public readonly Action<Filament.Chunk.DensityMapCollection> Callback;
                public readonly Filament.Chunk.DensityMapCollection DensityMaps;

                public FilamentDensityMapCoroutineInfo(Action<Filament.Chunk.DensityMapCollection> callback, Filament.Chunk.DensityMapCollection densityMaps)
                {
                    Callback = callback;
                    DensityMaps = densityMaps;
                }
            }

            private struct SectorDensityMapCoroutineInfo
            {
                public readonly Action<Sector.Chunk.DensityMapCollection> Callback;
                public readonly Sector.Chunk.DensityMapCollection DensityMaps;

                public SectorDensityMapCoroutineInfo(Action<Sector.Chunk.DensityMapCollection> callback, Sector.Chunk.DensityMapCollection densityMaps)
                {
                    Callback = callback;
                    DensityMaps = densityMaps;
                }
            }

            private struct RegionDensityMapCoroutineInfo
            {
                public readonly Action<Region.Chunk.DensityMapCollection> Callback;
                public readonly Region.Chunk.DensityMapCollection DensityMaps;

                public RegionDensityMapCoroutineInfo(Action<Region.Chunk.DensityMapCollection> callback, Region.Chunk.DensityMapCollection densityMaps)
                {
                    Callback = callback;
                    DensityMaps = densityMaps;
                }
            }

            private struct UniverseGenerationSettingsGPU
            {
                public readonly int Seed;
                public readonly int Size;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float Power;
                public readonly float Frequency;
                public readonly int Octaves;
                public readonly float Persistence;
                public readonly float Lacunarity;
                public readonly float Amplitude;
                public readonly float CellularJitter;

                public UniverseGenerationSettingsGPU(Universe.GenerationSettings settings)
                {
                    Seed = settings.Seed;
                    Size = settings.Size;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    Power = settings.Power;
                    Frequency = settings.Frequency;
                    Octaves = settings.Octaves;
                    Persistence = settings.Persistence;
                    Lacunarity = settings.Lacunarity;
                    Amplitude = settings.Amplitude;
                    CellularJitter = settings.CellularJitter;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) * 3 + sizeof(float) * 10;
                    }
                }
            }

            private struct FilamentGenerationSettingsGPU
            {
                public readonly int Seed;
                public readonly int ChunkSize;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float UniverseNoiseInfluence;
                public readonly float Power;
                public readonly float Frequency;
                public readonly int Octaves;
                public readonly float Persistence;
                public readonly float Lacunarity;
                public readonly float Amplitude;
                public readonly float CellularJitter;

                public FilamentGenerationSettingsGPU(Universe.Filament.GenerationSettings settings)
                {
                    Seed = settings.Seed;
                    ChunkSize = settings.ChunkSize;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    UniverseNoiseInfluence = settings.UniverseNoiseInfluence;
                    Power = settings.Power;
                    Frequency = settings.Frequency;
                    Octaves = settings.Octaves;
                    Persistence = settings.Persistence;
                    Lacunarity = settings.Lacunarity;
                    Amplitude = settings.Amplitude;
                    CellularJitter = settings.CellularJitter;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) * 3 + sizeof(float) * 11;
                    }
                }
            }

            private struct SectorGenerationSettingsGPU
            {
                public readonly int Seed;
                public readonly int ChunkSize;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float FilamentNoiseInfluence;
                public readonly float Power;
                public readonly float Frequency;
                public readonly int Octaves;
                public readonly float Persistence;
                public readonly float Lacunarity;
                public readonly float Amplitude;

                public SectorGenerationSettingsGPU(Universe.Sector.GenerationSettings settings)
                {
                    Seed = settings.Seed;
                    ChunkSize = settings.ChunkSize;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    FilamentNoiseInfluence = settings.FilamentNoiseInfluence;
                    Power = settings.Power;
                    Frequency = settings.Frequency;
                    Octaves = settings.Octaves;
                    Persistence = settings.Persistence;
                    Lacunarity = settings.Lacunarity;
                    Amplitude = settings.Amplitude;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) * 3 + sizeof(float) * 10;
                    }
                }
            }

            private struct RegionGenerationSettingsGPU
            {
                public readonly int Seed;
                public readonly int ChunkSize;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float SectorNoiseInfluence;
                public readonly float Power;
                public readonly float Frequency;
                public readonly int Octaves;
                public readonly float Persistence;
                public readonly float Lacunarity;
                public readonly float Amplitude;

                public RegionGenerationSettingsGPU(Universe.Region.GenerationSettings settings)
                {
                    Seed = settings.Seed;
                    ChunkSize = settings.ChunkSize;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    SectorNoiseInfluence = settings.SectorNoiseInfluence;
                    Power = settings.Power;
                    Frequency = settings.Frequency;
                    Octaves = settings.Octaves;
                    Persistence = settings.Persistence;
                    Lacunarity = settings.Lacunarity;
                    Amplitude = settings.Amplitude;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) * 3 + sizeof(float) * 10;
                    }
                }
            }

            private struct DensityDataGPU
            {
                public readonly int XPosition;
                public readonly int YPosition;
                public readonly float Value;

                public DensityDataGPU(int xPosition, int yPosition, float value)
                {
                    XPosition = xPosition;
                    YPosition = yPosition;
                    Value = value;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) * 2 + sizeof(float);
                    }
                }
            }
            #endregion

            #region Static Properties
            public static DensityMapGenerationUtil Instance { get; private set; }
            #endregion

            #region Static Fields
            private static Queue<FilamentDensityMapCoroutineInfo> filamentDensityMapCoroutineInfoQueue = new Queue<FilamentDensityMapCoroutineInfo>();
            private static Queue<SectorDensityMapCoroutineInfo> sectorDensityMapCoroutineInfoQueue = new Queue<SectorDensityMapCoroutineInfo>();
            private static Queue<RegionDensityMapCoroutineInfo> regionDensityMapCoroutineInfoQueue = new Queue<RegionDensityMapCoroutineInfo>();
            #endregion

            #region Fields
            private ComputeShader universeDensityShader;
            private ComputeShader filamentDensityShader;
            private ComputeShader sectorDensityShader;
            private ComputeShader regionDensityShader;
            #endregion

            #region Unity Callbacks
            private void Update()
            {
                ProcessFilamentDensityMapCoroutineInfoQueue();
                ProcessSectorDensityMapCoroutineInfoQueue();
                ProcessRegionDensityMapCoroutineInfoQueue();
            }
            #endregion

            #region Static Methods
            public static void InitializeInstance()
            {
                if (Instance != null)
                {
                    throw new Exception("Cannot have multiple instances of Universe.DensityMapGenerationUtil!");
                }
                GameObject instanceObject = new UnityEngine.GameObject("[Universe.DensityMapGenerationUtil]");
                instanceObject.layer = 31;
                instanceObject.tag = "INTERNAL";
                Instance = instanceObject.AddComponent<DensityMapGenerationUtil>();
                DontDestroyOnLoad(Instance);

                Instance.universeDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/UniverseDensity");
                Instance.filamentDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/FilamentDensity");
                Instance.sectorDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/SectorDensity");
                Instance.regionDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/RegionDensity");

                Instance.StartCoroutine(Instance.FilamentLoadQueueProcessingCoroutine());
                Instance.StartCoroutine(Instance.SectorLoadQueueProcessingCoroutine());
                Instance.StartCoroutine(Instance.RegionLoadQueueProcessingCoroutine());

                Debug.Log("[Universe.DensityMapGenerationUtil] Initialized.");
            }
            
            public static DensityMap GenerateUniverseDensityMap(GenerationSettings universeGenerationSettings)
            {
                if (Instance == null)
                {
                    throw new Exception("Universe.DensityMapGenerationUtil has not been initialized!");
                }
                
                UniverseGenerationSettingsGPU[] universeGenerationSettingsData = { new UniverseGenerationSettingsGPU(universeGenerationSettings) };
                DensityDataGPU[] universeDensitiesData = new DensityDataGPU[universeGenerationSettings.Size * universeGenerationSettings.Size];

                for (int x = 0; x < universeGenerationSettings.Size; x++)
                {
                    for (int y = 0; y < universeGenerationSettings.Size; y++)
                    {
                        int index = x * universeGenerationSettings.Size + y;
                        universeDensitiesData[index] = new DensityDataGPU(x, y, 0);
                    }
                }

                ComputeBuffer universeGenerationSettingsBuffer = new ComputeBuffer(1, UniverseGenerationSettingsGPU.ByteSize);
                universeGenerationSettingsBuffer.SetData(universeGenerationSettingsData);
                ComputeBuffer universeDensitiesBuffer = new ComputeBuffer(universeDensitiesData.Length, DensityDataGPU.ByteSize);
                universeDensitiesBuffer.SetData(universeDensitiesData);

                Instance.universeDensityShader.SetBuffer(0, "universeGenerationSettingsBuffer", universeGenerationSettingsBuffer);
                Instance.universeDensityShader.SetBuffer(0, "universeDensityMap", universeDensitiesBuffer);

                Instance.universeDensityShader.Dispatch(0, universeGenerationSettings.Size, universeGenerationSettings.Size, 1);

                universeDensitiesBuffer.GetData(universeDensitiesData);

                SerializableDictionary<Vector2Int, float> universeDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();

                for (int x = 0; x < universeGenerationSettings.Size; x++)
                {
                    for (int y = 0; y < universeGenerationSettings.Size; y++)
                    {
                        int index = x * universeGenerationSettings.Size + y;
                        universeDensityMapDictionary.Add(new Vector2Int(x, y), universeDensitiesData[index].Value);
                    }
                }

                universeGenerationSettingsBuffer.Dispose();
                universeDensitiesBuffer.Dispose();

                return new DensityMap(universeDensityMapDictionary);
            }

            public static void RequestGenerateFilamentDensityMaps(DensityMap universeDensityMap, Filament.Chunk.DensityMapCollection filamentDensityMaps, Action<Filament.Chunk.DensityMapCollection> callback)
            {
                if (Instance == null)
                {
                    throw new Exception("Universe.ParallelizationUtil has not been initialized!");
                }

                Instance.StartCoroutine(Instance.FilamentDensityMapsGenerationCoroutine(universeDensityMap, filamentDensityMaps, callback));
            }

            public static void RequestGenerateSectorDensityMaps(Filament.Chunk.DensityMapCollection filamentDensityMaps, Sector.Chunk.DensityMapCollection sectorDensityMaps, Action<Sector.Chunk.DensityMapCollection> callback)
            {
                if (Instance == null)
                {
                    throw new Exception("Universe.ParallelizationUtil has not been initialized!");
                }

                Instance.StartCoroutine(Instance.SectorDensityMapsGenerationCoroutine(filamentDensityMaps, sectorDensityMaps, callback));
            }

            public static void RequestRegionDensityMapsGeneration(Sector.Chunk.DensityMapCollection sectorDensityMaps, Region.Chunk.DensityMapCollection regionDensityMaps, Action<Region.Chunk.DensityMapCollection> callback)
            {
                if (Instance == null)
                {
                    throw new Exception("Universe.ParallelizationUtil has not been initialized!");
                }

                Instance.StartCoroutine(Instance.RegionDensityMapsGenerationCoroutine(sectorDensityMaps, regionDensityMaps, callback));
            }

            public static void ProcessFilamentDensityMapCoroutineInfoQueue()
            {
                while (filamentDensityMapCoroutineInfoQueue.Count > 0)
                {
                    FilamentDensityMapCoroutineInfo filamentDensityMapCoroutineInfo = filamentDensityMapCoroutineInfoQueue.Dequeue();
                    filamentDensityMapCoroutineInfo.Callback(filamentDensityMapCoroutineInfo.DensityMaps);
                }
            }
            
            public static void ProcessSectorDensityMapCoroutineInfoQueue()
            {
                while (sectorDensityMapCoroutineInfoQueue.Count > 0)
                {
                    SectorDensityMapCoroutineInfo sectorDensityMapCoroutineInfo = sectorDensityMapCoroutineInfoQueue.Dequeue();
                    sectorDensityMapCoroutineInfo.Callback(sectorDensityMapCoroutineInfo.DensityMaps);
                }
            }

            public static void ProcessRegionDensityMapCoroutineInfoQueue()
            {
                while (regionDensityMapCoroutineInfoQueue.Count > 0)
                {
                    RegionDensityMapCoroutineInfo regionDensityMapCoroutineInfo = regionDensityMapCoroutineInfoQueue.Dequeue();
                    regionDensityMapCoroutineInfo.Callback(regionDensityMapCoroutineInfo.DensityMaps);
                }
            }
            #endregion

            #region Coroutines
            // TODO: Parallelize for-Loops to better utilize multithreading
            private IEnumerator FilamentDensityMapsGenerationCoroutine(DensityMap universeDensityMap, Filament.Chunk.DensityMapCollection filamentDensityMaps, Action<Filament.Chunk.DensityMapCollection> callback)
            {
                GenerationSettings universeGenerationSettings = GameManager.Instance.CurrentGame.CurrentUniverse.UniverseGenerationSettings;
                Filament.GenerationSettings filamentGenerationSettings = universeGenerationSettings.FilamentGenerationSettings;
                
                FilamentGenerationSettingsGPU[] filamentGenerationSettingsData = { new FilamentGenerationSettingsGPU(filamentGenerationSettings) };
                
                DensityDataGPU[] universeDensitiesData = new DensityDataGPU[universeDensityMap.DensityMapDictionary.Count];
                
                DensityDataGPU[] electronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] positronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] protonDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] antiProtonDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] neutronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] antiNeutronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];

                for (int x = 0; x < universeGenerationSettings.Size; x++)
                {
                    for (int y = 0; y < universeGenerationSettings.Size; y++)
                    {
                        int index = x * universeGenerationSettings.Size + y;
                        universeDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                    }
                }

                for (int x = 0; x < filamentGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < filamentGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * filamentGenerationSettings.ChunkSize + y;
                        electronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        positronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        protonDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        antiProtonDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        neutronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        antiNeutronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                    }
                }

                ComputeBuffer filamentGenerationSettingsBuffer = new ComputeBuffer(1, FilamentGenerationSettingsGPU.ByteSize);
                filamentGenerationSettingsBuffer.SetData(filamentGenerationSettingsData);
                ComputeBuffer universeDensitiesBuffer = new ComputeBuffer(universeDensitiesData.Length, DensityDataGPU.ByteSize);
                universeDensitiesBuffer.SetData(universeDensitiesData);
                ComputeBuffer electronDensityBuffer = new ComputeBuffer(electronDensitiesData.Length, DensityDataGPU.ByteSize);
                electronDensityBuffer.SetData(electronDensitiesData);
                ComputeBuffer positronDensityBuffer = new ComputeBuffer(positronDensitiesData.Length, DensityDataGPU.ByteSize);
                positronDensityBuffer.SetData(positronDensitiesData);
                ComputeBuffer protonDensityBuffer = new ComputeBuffer(protonDensitiesData.Length, DensityDataGPU.ByteSize);
                protonDensityBuffer.SetData(protonDensitiesData);
                ComputeBuffer antiProtonDensityBuffer = new ComputeBuffer(antiProtonDensitiesData.Length, DensityDataGPU.ByteSize);
                antiProtonDensityBuffer.SetData(antiProtonDensitiesData);
                ComputeBuffer neutronDensityBuffer = new ComputeBuffer(neutronDensitiesData.Length, DensityDataGPU.ByteSize);
                neutronDensityBuffer.SetData(neutronDensitiesData);
                ComputeBuffer antiNeutronDensityBuffer = new ComputeBuffer(antiNeutronDensitiesData.Length, DensityDataGPU.ByteSize);
                antiNeutronDensityBuffer.SetData(antiNeutronDensitiesData);

                filamentDensityShader.SetBuffer(0, "filamentGenerationSettingsBuffer", filamentGenerationSettingsBuffer);
                filamentDensityShader.SetBuffer(0, "universeDensityMap", universeDensitiesBuffer);
                filamentDensityShader.SetBuffer(0, "electronDensityMap", electronDensityBuffer);
                filamentDensityShader.SetBuffer(0, "positronDensityMap", positronDensityBuffer);
                filamentDensityShader.SetBuffer(0, "protonDensityMap", protonDensityBuffer);
                filamentDensityShader.SetBuffer(0, "antiProtonDensityMap", antiProtonDensityBuffer);
                filamentDensityShader.SetBuffer(0, "neutronDensityMap", neutronDensityBuffer);
                filamentDensityShader.SetBuffer(0, "antiNeutronDensityMap", antiNeutronDensityBuffer);
                
                filamentDensityShader.Dispatch(0, filamentGenerationSettings.ChunkSize, filamentGenerationSettings.ChunkSize, 1);

                electronDensityBuffer.GetData(electronDensitiesData);
                positronDensityBuffer.GetData(positronDensitiesData);
                protonDensityBuffer.GetData(protonDensitiesData);
                antiProtonDensityBuffer.GetData(antiProtonDensitiesData);
                neutronDensityBuffer.GetData(neutronDensitiesData);
                antiNeutronDensityBuffer.GetData(antiNeutronDensitiesData);

                SerializableDictionary<Vector2Int, float> electronDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> positronDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> protonDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> antiProtonDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> neutronDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> antiNeutronDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();

                for (int x = 0; x < filamentGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < filamentGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * filamentGenerationSettings.ChunkSize + y;
                        electronDensityMapDictionary.Add(new Vector2Int(x, y), electronDensitiesData[index].Value);
                        positronDensityMapDictionary.Add(new Vector2Int(x, y), positronDensitiesData[index].Value);
                        protonDensityMapDictionary.Add(new Vector2Int(x, y), protonDensitiesData[index].Value);
                        antiProtonDensityMapDictionary.Add(new Vector2Int(x, y), antiProtonDensitiesData[index].Value);
                        neutronDensityMapDictionary.Add(new Vector2Int(x, y), neutronDensitiesData[index].Value);
                        antiNeutronDensityMapDictionary.Add(new Vector2Int(x, y), antiNeutronDensitiesData[index].Value);
                    }
                }
                
                filamentDensityMaps.ElectronDensityMap = new Filament.Chunk.DensityMap(electronDensityMapDictionary, Filament.Chunk.DensityMapType.Electron);
                filamentDensityMaps.PositronDensityMap = new Filament.Chunk.DensityMap(positronDensityMapDictionary, Filament.Chunk.DensityMapType.Positron);
                filamentDensityMaps.ProtonDensityMap = new Filament.Chunk.DensityMap(protonDensityMapDictionary, Filament.Chunk.DensityMapType.Proton);
                filamentDensityMaps.AntiProtonDensityMap = new Filament.Chunk.DensityMap(antiProtonDensityMapDictionary, Filament.Chunk.DensityMapType.AntiProton);
                filamentDensityMaps.NeutronDensityMap = new Filament.Chunk.DensityMap(neutronDensityMapDictionary, Filament.Chunk.DensityMapType.Neutron);
                filamentDensityMaps.AntiNeutronDensityMap = new Filament.Chunk.DensityMap(antiNeutronDensityMapDictionary, Filament.Chunk.DensityMapType.AntiNeutron);
                filamentDensityMaps.GenerationState = Filament.Chunk.GenerationState.Generated;

                filamentGenerationSettingsBuffer.Dispose();
                universeDensitiesBuffer.Dispose();
                electronDensityBuffer.Dispose();
                positronDensityBuffer.Dispose();
                protonDensityBuffer.Dispose();
                antiProtonDensityBuffer.Dispose();
                neutronDensityBuffer.Dispose();
                antiNeutronDensityBuffer.Dispose();

                filamentDensityMapCoroutineInfoQueue.Enqueue(new FilamentDensityMapCoroutineInfo(callback, filamentDensityMaps));
                yield return null;
            }

            // TODO: Parallelize for-Loops to better utilize multithreading
            private IEnumerator SectorDensityMapsGenerationCoroutine(Filament.Chunk.DensityMapCollection filamentDensityMaps, Sector.Chunk.DensityMapCollection sectorDensityMaps, Action<Sector.Chunk.DensityMapCollection> callback)
            {
                GenerationSettings universeGenerationSettings = GameManager.Instance.CurrentGame.CurrentUniverse.UniverseGenerationSettings;
                Sector.GenerationSettings sectorGenerationSettings = universeGenerationSettings.SectorGenerationSettings;
                Filament.GenerationSettings filamentGenerationSettings = universeGenerationSettings.FilamentGenerationSettings;
                
                SectorGenerationSettingsGPU[] sectorGenerationSettingsData = { new SectorGenerationSettingsGPU(sectorGenerationSettings) };
                
                DensityDataGPU[] solidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                DensityDataGPU[] liquidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                DensityDataGPU[] gasParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                DensityDataGPU[] plasmaParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                
                DensityDataGPU[] electronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] positronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] protonDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] antiProtonDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] neutronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];
                DensityDataGPU[] antiNeutronDensitiesData = new DensityDataGPU[filamentDensityMaps.Size * filamentDensityMaps.Size];

                for (int x = 0; x < sectorGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < sectorGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * sectorGenerationSettings.ChunkSize + y;
                        solidParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        liquidParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        gasParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        plasmaParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                    }
                }

                for (int x = 0; x < filamentGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < filamentGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * filamentGenerationSettings.ChunkSize + y;
                        electronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        positronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        protonDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        antiProtonDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        neutronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        antiNeutronDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                    }
                }

                ComputeBuffer sectorGenerationSettingsBuffer = new ComputeBuffer(1, SectorGenerationSettingsGPU.ByteSize);
                sectorGenerationSettingsBuffer.SetData(sectorGenerationSettingsData);
                ComputeBuffer solidParticleDensityBuffer = new ComputeBuffer(solidParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                solidParticleDensityBuffer.SetData(solidParticleDensitiesData);
                ComputeBuffer liquidParticleDensityBuffer = new ComputeBuffer(liquidParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                liquidParticleDensityBuffer.SetData(liquidParticleDensitiesData);
                ComputeBuffer gasParticleDensityBuffer = new ComputeBuffer(gasParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                gasParticleDensityBuffer.SetData(gasParticleDensitiesData);
                ComputeBuffer plasmaParticleDensityBuffer = new ComputeBuffer(plasmaParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                plasmaParticleDensityBuffer.SetData(plasmaParticleDensitiesData);
                ComputeBuffer electronDensityBuffer = new ComputeBuffer(electronDensitiesData.Length, DensityDataGPU.ByteSize);
                electronDensityBuffer.SetData(electronDensitiesData);
                ComputeBuffer positronDensityBuffer = new ComputeBuffer(positronDensitiesData.Length, DensityDataGPU.ByteSize);
                positronDensityBuffer.SetData(positronDensitiesData);
                ComputeBuffer protonDensityBuffer = new ComputeBuffer(protonDensitiesData.Length, DensityDataGPU.ByteSize);
                protonDensityBuffer.SetData(protonDensitiesData);
                ComputeBuffer antiProtonDensityBuffer = new ComputeBuffer(antiProtonDensitiesData.Length, DensityDataGPU.ByteSize);
                antiProtonDensityBuffer.SetData(antiProtonDensitiesData);
                ComputeBuffer neutronDensityBuffer = new ComputeBuffer(neutronDensitiesData.Length, DensityDataGPU.ByteSize);
                neutronDensityBuffer.SetData(neutronDensitiesData);
                ComputeBuffer antiNeutronDensityBuffer = new ComputeBuffer(antiNeutronDensitiesData.Length, DensityDataGPU.ByteSize);
                antiNeutronDensityBuffer.SetData(antiNeutronDensitiesData);
                
                sectorDensityShader.SetBuffer(0, "sectorGenerationSettingsBuffer", sectorGenerationSettingsBuffer);
                sectorDensityShader.SetBuffer(0, "solidParticleDensityMap", solidParticleDensityBuffer);
                sectorDensityShader.SetBuffer(0, "liquidParticleDensityMap", liquidParticleDensityBuffer);
                sectorDensityShader.SetBuffer(0, "gasParticleDensityMap", gasParticleDensityBuffer);
                sectorDensityShader.SetBuffer(0, "plasmaParticleDensityMap", plasmaParticleDensityBuffer);
                sectorDensityShader.SetBuffer(0, "electronDensityMap", electronDensityBuffer);
                sectorDensityShader.SetBuffer(0, "positronDensityMap", positronDensityBuffer);
                sectorDensityShader.SetBuffer(0, "protonDensityMap", protonDensityBuffer);
                sectorDensityShader.SetBuffer(0, "antiProtonDensityMap", antiProtonDensityBuffer);
                sectorDensityShader.SetBuffer(0, "neutronDensityMap", neutronDensityBuffer);
                sectorDensityShader.SetBuffer(0, "antiNeutronDensityMap", antiNeutronDensityBuffer);

                sectorDensityShader.Dispatch(0, sectorGenerationSettings.ChunkSize, sectorGenerationSettings.ChunkSize, 1);

                solidParticleDensityBuffer.GetData(electronDensitiesData);
                liquidParticleDensityBuffer.GetData(positronDensitiesData);
                gasParticleDensityBuffer.GetData(protonDensitiesData);
                plasmaParticleDensityBuffer.GetData(antiProtonDensitiesData);

                SerializableDictionary<Vector2Int, float> solidParticleDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> liquidParticleDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> gasParticleDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> plasmaParticleDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();

                for (int x = 0; x < sectorGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < sectorGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * sectorGenerationSettings.ChunkSize + y;
                        solidParticleDensityMapDictionary.Add(new Vector2Int(x, y), solidParticleDensitiesData[index].Value);
                        liquidParticleDensityMapDictionary.Add(new Vector2Int(x, y), liquidParticleDensitiesData[index].Value);
                        gasParticleDensityMapDictionary.Add(new Vector2Int(x, y), gasParticleDensitiesData[index].Value);
                        plasmaParticleDensityMapDictionary.Add(new Vector2Int(x, y), plasmaParticleDensitiesData[index].Value);
                    }
                }

                sectorDensityMaps.SolidParticleDensityMap = new Sector.Chunk.DensityMap(solidParticleDensityMapDictionary, Sector.Chunk.DensityMapType.SolidParticle);
                sectorDensityMaps.LiquidParticleDensityMap = new Sector.Chunk.DensityMap(liquidParticleDensityMapDictionary, Sector.Chunk.DensityMapType.LiquidParticle);
                sectorDensityMaps.GasParticleDensityMap = new Sector.Chunk.DensityMap(gasParticleDensityMapDictionary, Sector.Chunk.DensityMapType.GasParticle);
                sectorDensityMaps.PlasmaParticleDensityMap = new Sector.Chunk.DensityMap(plasmaParticleDensityMapDictionary, Sector.Chunk.DensityMapType.PlasmaParticle);
                sectorDensityMaps.GenerationState = Sector.Chunk.GenerationState.Generated;

                sectorGenerationSettingsBuffer.Dispose();
                solidParticleDensityBuffer.Dispose();
                liquidParticleDensityBuffer.Dispose();
                gasParticleDensityBuffer.Dispose();
                plasmaParticleDensityBuffer.Dispose();
                electronDensityBuffer.Dispose();
                positronDensityBuffer.Dispose();
                protonDensityBuffer.Dispose();
                antiProtonDensityBuffer.Dispose();
                neutronDensityBuffer.Dispose();
                antiNeutronDensityBuffer.Dispose();

                sectorDensityMapCoroutineInfoQueue.Enqueue(new SectorDensityMapCoroutineInfo(callback, sectorDensityMaps));
                yield return null;
            }

            // TODO: Parallelize for-Loops to better utilize multithreading
            private IEnumerator RegionDensityMapsGenerationCoroutine(Sector.Chunk.DensityMapCollection sectorDensityMaps, Region.Chunk.DensityMapCollection regionDensityMaps, Action<Region.Chunk.DensityMapCollection> callback)
            {
                GenerationSettings universeGenerationSettings = GameManager.Instance.CurrentGame.CurrentUniverse.UniverseGenerationSettings;
                Region.GenerationSettings regionGenerationSettings = universeGenerationSettings.RegionGenerationSettings;
                Sector.GenerationSettings sectorGenerationSettings = universeGenerationSettings.SectorGenerationSettings;
                
                RegionGenerationSettingsGPU[] regionGenerationSettingsData = { new RegionGenerationSettingsGPU(regionGenerationSettings) };
                
                DensityDataGPU[] matterParticleDensitiesData = new DensityDataGPU[regionDensityMaps.Size * regionDensityMaps.Size];
                DensityDataGPU[] antiMatterParticleDensitiesData = new DensityDataGPU[regionDensityMaps.Size * regionDensityMaps.Size];

                DensityDataGPU[] solidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                DensityDataGPU[] liquidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                DensityDataGPU[] gasParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];
                DensityDataGPU[] plasmaParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.Size * sectorDensityMaps.Size];

                for (int x = 0; x < regionGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < regionGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * regionGenerationSettings.ChunkSize + y;
                        matterParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        antiMatterParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                    }
                }

                for (int x = 0; x < sectorGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < sectorGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * sectorGenerationSettings.ChunkSize + y;
                        solidParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        liquidParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        gasParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                        plasmaParticleDensitiesData[index] = new DensityDataGPU(x, y, 0.0f);
                    }
                }

                ComputeBuffer regionGenerationSettingsBuffer = new ComputeBuffer(1, RegionGenerationSettingsGPU.ByteSize);
                regionGenerationSettingsBuffer.SetData(regionGenerationSettingsData);
                ComputeBuffer matterParticleDensityBuffer = new ComputeBuffer(matterParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                matterParticleDensityBuffer.SetData(matterParticleDensitiesData);
                ComputeBuffer antiMatterParticleDensityBuffer = new ComputeBuffer(antiMatterParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                antiMatterParticleDensityBuffer.SetData(antiMatterParticleDensitiesData);
                ComputeBuffer solidParticleDensityBuffer = new ComputeBuffer(solidParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                solidParticleDensityBuffer.SetData(solidParticleDensitiesData);
                ComputeBuffer liquidParticleDensityBuffer = new ComputeBuffer(liquidParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                liquidParticleDensityBuffer.SetData(liquidParticleDensitiesData);
                ComputeBuffer gasParticleDensityBuffer = new ComputeBuffer(gasParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                gasParticleDensityBuffer.SetData(gasParticleDensitiesData);
                ComputeBuffer plasmaParticleDensityBuffer = new ComputeBuffer(plasmaParticleDensitiesData.Length, DensityDataGPU.ByteSize);
                plasmaParticleDensityBuffer.SetData(plasmaParticleDensitiesData);

                regionDensityShader.SetBuffer(0, "regionGenerationSettingsBuffer", regionGenerationSettingsBuffer);
                regionDensityShader.SetBuffer(0, "solidParticleDensityMap", solidParticleDensityBuffer);
                regionDensityShader.SetBuffer(0, "liquidParticleDensityMap", liquidParticleDensityBuffer);
                regionDensityShader.SetBuffer(0, "gasParticleDensityMap", gasParticleDensityBuffer);
                regionDensityShader.SetBuffer(0, "plasmaParticleDensityMap", plasmaParticleDensityBuffer);
                regionDensityShader.SetBuffer(0, "matterParticleDensityMap", matterParticleDensityBuffer);
                regionDensityShader.SetBuffer(0, "antiMatterParticleDensityMap", antiMatterParticleDensityBuffer);

                regionDensityShader.Dispatch(0, regionGenerationSettings.ChunkSize, regionGenerationSettings.ChunkSize, 1);

                matterParticleDensityBuffer.GetData(matterParticleDensitiesData);
                antiMatterParticleDensityBuffer.GetData(antiMatterParticleDensitiesData);

                SerializableDictionary<Vector2Int, float> matterParticleDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();
                SerializableDictionary<Vector2Int, float> antiMatterParticleDensityMapDictionary = new SerializableDictionary<Vector2Int, float>();

                for (int x = 0; x < regionGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < regionGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * regionGenerationSettings.ChunkSize + y;
                        matterParticleDensityMapDictionary.Add(new Vector2Int(x, y), matterParticleDensitiesData[index].Value);
                        antiMatterParticleDensityMapDictionary.Add(new Vector2Int(x, y), antiMatterParticleDensitiesData[index].Value);
                    }
                }

                regionDensityMaps.MatterDensityMap = new Region.Chunk.DensityMap(matterParticleDensityMapDictionary, Region.Chunk.DensityMapType.Matter);
                regionDensityMaps.AntiMatterDensityMap = new Region.Chunk.DensityMap(antiMatterParticleDensityMapDictionary, Region.Chunk.DensityMapType.AntiMatter);
                regionDensityMaps.GenerationState = Region.Chunk.GenerationState.Generated;

                regionGenerationSettingsBuffer.Dispose();
                matterParticleDensityBuffer.Dispose();
                antiMatterParticleDensityBuffer.Dispose();
                solidParticleDensityBuffer.Dispose();
                liquidParticleDensityBuffer.Dispose();
                gasParticleDensityBuffer.Dispose();
                plasmaParticleDensityBuffer.Dispose();

                regionDensityMapCoroutineInfoQueue.Enqueue(new RegionDensityMapCoroutineInfo(callback, regionDensityMaps));
                yield return null;
            }
            
            private IEnumerator FilamentLoadQueueProcessingCoroutine()
            {
                while (true)
                {
                    while (Filament.Chunk.DensityMapCoroutineInfoQueue.Count > 0)
                    {
                        Filament.Chunk.DensityMapCoroutineInfo threadInfo = Filament.Chunk.DensityMapCoroutineInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);

                        yield return null;
                    }
                    
                    yield return null;
                }
            }

            private IEnumerator SectorLoadQueueProcessingCoroutine()
            {
                while (true)
                {
                    while (Sector.Chunk.DensityMapCoroutineInfoQueue.Count > 0)
                    {
                        Sector.Chunk.DensityMapCoroutineInfo threadInfo = Sector.Chunk.DensityMapCoroutineInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);

                        yield return null;
                    }
                    
                    yield return null;
                }
            }

            private IEnumerator RegionLoadQueueProcessingCoroutine()
            {
                while (true)
                {
                    while (Region.Chunk.DensityMapCoroutineInfoQueue.Count > 0)
                    {
                        Region.Chunk.DensityMapCoroutineInfo threadInfo = Region.Chunk.DensityMapCoroutineInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);

                        yield return null;
                    }
                    
                    yield return null;
                }
            }
            #endregion
        }

        public class MapElementLoadingUtil : MonoBehaviour
        {
            #region Static Properties
            public static MapElementLoadingUtil Instance { get; private set; }
            public static List<Filament.Position> FilamentLoadRequests => filamentLoadRequests;
            public static List<Sector.Position> SectorLoadRequests => sectorLoadRequests;
            public static List<Region.Position> RegionLoadRequests => regionLoadRequests;
            public static List<Filament.Chunk.Position> FilamentChunkLoadRequests => filamentChunkLoadRequests;
            public static List<Sector.Chunk.Position> SectorChunkLoadRequests => sectorChunkLoadRequests;
            public static List<Region.Chunk.Position> RegionChunkLoadRequests => regionChunkLoadRequests;
            #endregion

            #region Static Fields
            private static List<Filament.Position> filamentLoadRequests;
            private static List<Sector.Position> sectorLoadRequests;
            private static List<Region.Position> regionLoadRequests;
            private static List<Filament.Chunk.Position> filamentChunkLoadRequests;
            private static List<Sector.Chunk.Position> sectorChunkLoadRequests;
            private static List<Region.Chunk.Position> regionChunkLoadRequests;
            #endregion

            #region Fields
            private Universe currentUniverse;
            #endregion

            #region Static Methods
            public static void InitializeInstance()
            {
                if (Instance != null)
                {
                    throw new Exception("Cannot have multiple instances of Universe.MapElementLoadingUtil!");
                }
                GameObject instanceObject = new UnityEngine.GameObject("[Universe.MapElementLoadingUtil]");
                instanceObject.layer = 31;
                instanceObject.tag = "INTERNAL";
                Instance = instanceObject.AddComponent<MapElementLoadingUtil>();
                DontDestroyOnLoad(Instance);

                filamentLoadRequests = new List<Filament.Position>();
                sectorLoadRequests = new List<Sector.Position>();
                regionLoadRequests = new List<Region.Position>();
                filamentChunkLoadRequests = new List<Filament.Chunk.Position>();
                sectorChunkLoadRequests = new List<Sector.Chunk.Position>();
                regionChunkLoadRequests = new List<Region.Chunk.Position>();

                Instance.currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
                
                Instance.StartCoroutine(Instance.ProcessFilamentLoadQueueCoroutine());
                Instance.StartCoroutine(Instance.ProcessFilamentChunkLoadQueueCoroutine());
                Instance.StartCoroutine(Instance.ProcessSectorLoadQueueCoroutine());
                Instance.StartCoroutine(Instance.ProcessSectorChunkLoadQueueCoroutine());
                Instance.StartCoroutine(Instance.ProcessRegionLoadQueueCoroutine());
                Instance.StartCoroutine(Instance.ProcessRegionChunkLoadQueueCoroutine());

                Debug.Log("[Universe.MapElementLoadingUtil] Initialized.");
            }

            public static void RequestFilamentLoad(Filament.Position filamentPosition)
            {
                if (!filamentLoadRequests.Contains(filamentPosition))
                {
                    filamentLoadRequests.Add(filamentPosition);
                }
            }

            public static void RequestFilamentChunkLoad(Filament.Chunk.Position filamentChunkPosition)
            {
                if (!filamentChunkLoadRequests.Contains(filamentChunkPosition))
                {
                    filamentChunkLoadRequests.Add(filamentChunkPosition);
                }
            }

            public static void RequestSectorLoad(Sector.Position sectorPosition)
            {
                if (!sectorLoadRequests.Contains(sectorPosition))
                {
                    sectorLoadRequests.Add(sectorPosition);
                }
            }

            public static void RequestSectorChunkLoad(Sector.Chunk.Position sectorChunkPosition)
            {
                if (!sectorChunkLoadRequests.Contains(sectorChunkPosition))
                {
                    sectorChunkLoadRequests.Add(sectorChunkPosition);
                }
            }

            public static void RequestRegionLoad(Region.Position regionPosition)
            {
                if (!regionLoadRequests.Contains(regionPosition))
                {
                    regionLoadRequests.Add(regionPosition);
                }
            }

            public static void RequestRegionChunkLoad(Region.Chunk.Position regionChunkPosition)
            {
                if (!regionChunkLoadRequests.Contains(regionChunkPosition))
                {
                    regionChunkLoadRequests.Add(regionChunkPosition);
                }
            }

            // TODO: Incorporate Cancellation Methods into Universe API in Generation as CancelFilament*GenerationRequest

            public static void CancelFilamentLoadRequest(Filament.Position filamentPosition)
            {
                if (filamentLoadRequests.Contains(filamentPosition))
                {
                    filamentLoadRequests.Remove(filamentPosition);
                }
            }

            public static void CancelFilamentChunkLoadRequest(Filament.Chunk.Position filamentChunkPosition)
            {
                if (filamentChunkLoadRequests.Contains(filamentChunkPosition))
                {
                    filamentChunkLoadRequests.Remove(filamentChunkPosition);
                }
            }

            public static void CancelSectorLoadRequest(Sector.Position sectorPosition)
            {
                if (sectorLoadRequests.Contains(sectorPosition))
                {
                    sectorLoadRequests.Remove(sectorPosition);
                }
            }

            public static void CancelSectorChunkLoadRequest(Sector.Chunk.Position sectorChunkPosition)
            {
                if (sectorChunkLoadRequests.Contains(sectorChunkPosition))
                {
                    sectorChunkLoadRequests.Remove(sectorChunkPosition);
                }
            }

            public static void CancelRegionLoadRequest(Region.Position regionPosition)
            {
                if (regionLoadRequests.Contains(regionPosition))
                {
                    regionLoadRequests.Remove(regionPosition);
                }
            }

            public static void CancelRegionChunkLoadRequest(Region.Chunk.Position regionChunkPosition)
            {
                if (regionChunkLoadRequests.Contains(regionChunkPosition))
                {
                    regionChunkLoadRequests.Remove(regionChunkPosition);
                }
            }
            #endregion

            #region Coroutines
            private IEnumerator ProcessFilamentLoadQueueCoroutine()
            {
                while (true)
                {
                    if (filamentLoadRequests.Count > 0)
                    {
                        List<Filament.Position> loadedFilamentPositions = new List<Filament.Position>();
                        List<Filament.Position> filamentLoadRequestsCopy = new List<Filament.Position>(filamentLoadRequests);
                        foreach (Filament.Position filamentPosition in filamentLoadRequestsCopy)
                        {
                            Filament filament = new Filament(currentUniverse, filamentPosition);
                            currentUniverse.loadedFilaments.Add(filamentPosition, filament);
                            currentUniverse.SaveFilament(filament);
                            loadedFilamentPositions.Add(filamentPosition);

                            yield return null;
                        }

                        foreach (Filament.Position filamentPosition in loadedFilamentPositions)
                        {
                            filamentLoadRequests.Remove(filamentPosition);
                        }

                        yield return null;
                    }
                    
                    yield return null;
                }
            }

            private IEnumerator ProcessFilamentChunkLoadQueueCoroutine()
            {
                while (true)
                {
                    if (filamentChunkLoadRequests.Count > 0)
                    {
                        List<Filament.Chunk.Position> loadedFilamentChunkPositions = new List<Filament.Chunk.Position>();
                        List<Filament.Chunk.Position> filamentChunkLoadRequestsCopy = new List<Filament.Chunk.Position>(filamentChunkLoadRequests);
                        foreach (Filament.Chunk.Position filamentChunkPosition in filamentChunkLoadRequestsCopy)
                        {
                            if (!currentUniverse.IsFilamentLoaded(filamentChunkPosition.FilamentPosition))
                            {
                                continue;
                            }

                            Filament filament = currentUniverse.GetFilament(filamentChunkPosition.FilamentPosition);
                            Filament.Chunk filamentChunk = new Filament.Chunk(currentUniverse, filament, filamentChunkPosition);
                            filament.RegisterChunkPosition(filamentChunkPosition);
                            currentUniverse.loadedFilamentChunks.Add(filamentChunkPosition, filamentChunk);
                            currentUniverse.SaveFilament(filament);
                            currentUniverse.SaveFilamentChunk(filamentChunk);
                            loadedFilamentChunkPositions.Add(filamentChunkPosition);

                            yield return null;
                        }

                        foreach (Filament.Chunk.Position filamentChunkPosition in loadedFilamentChunkPositions)
                        {
                            filamentChunkLoadRequests.Remove(filamentChunkPosition);
                        }

                        yield return null;
                    }

                    yield return null;
                }
            }

            private IEnumerator ProcessSectorLoadQueueCoroutine()
            {
                while (true)
                {
                    if (sectorLoadRequests.Count > 0)
                    {
                        List<Sector.Position> loadedSectorPositions = new List<Sector.Position>();
                        List<Sector.Position> sectorLoadRequestsCopy = new List<Sector.Position>(sectorLoadRequests);
                        foreach (Sector.Position sectorPosition in sectorLoadRequestsCopy)
                        {
                            if (!currentUniverse.IsFilamentLoaded(sectorPosition.FilamentPosition))
                            {
                                continue;
                            }

                            Filament filament = currentUniverse.GetFilament(sectorPosition.FilamentPosition);
                            Sector sector = new Sector(currentUniverse, sectorPosition);
                            filament.RegisterSectorPosition(sectorPosition);
                            currentUniverse.loadedSectors.Add(sectorPosition, sector);
                            currentUniverse.SaveFilament(filament);
                            currentUniverse.SaveSector(sector);
                            loadedSectorPositions.Add(sectorPosition);

                            yield return null;
                        }

                        foreach (Sector.Position sectorPosition in loadedSectorPositions)
                        {
                            sectorLoadRequests.Remove(sectorPosition);
                        }

                        yield return null;
                    }

                    yield return null;
                }
            }

            private IEnumerator ProcessSectorChunkLoadQueueCoroutine()
            {
                while (true)
                {
                    if (sectorChunkLoadRequests.Count > 0)
                    {
                        List<Sector.Chunk.Position> loadedSectorChunkPositions = new List<Sector.Chunk.Position>();
                        List<Sector.Chunk.Position> sectorChunkLoadRequestsCopy = new List<Sector.Chunk.Position>(sectorChunkLoadRequests);
                        foreach (Sector.Chunk.Position sectorChunkPosition in sectorChunkLoadRequestsCopy)
                        {
                            if (!currentUniverse.IsSectorLoaded(sectorChunkPosition.SectorPosition))
                            {
                                continue;
                            }

                            Sector sector = currentUniverse.GetSector(sectorChunkPosition.SectorPosition);
                            Filament filament = currentUniverse.GetFilament(sectorChunkPosition.FilamentPosition);
                            Sector.Chunk sectorChunk = new Sector.Chunk(currentUniverse, filament, sector, sectorChunkPosition);
                            sector.RegisterChunkPosition(sectorChunkPosition);
                            currentUniverse.loadedSectorChunks.Add(sectorChunkPosition, sectorChunk);
                            currentUniverse.SaveSector(sector);
                            currentUniverse.SaveSectorChunk(sectorChunk);
                            loadedSectorChunkPositions.Add(sectorChunkPosition);

                            yield return null;
                        }

                        foreach (Sector.Chunk.Position sectorChunkPosition in loadedSectorChunkPositions)
                        {
                            sectorChunkLoadRequests.Remove(sectorChunkPosition);
                        }

                        yield return null;
                    }

                    yield return null;
                }
            }

            private IEnumerator ProcessRegionLoadQueueCoroutine()
            {
                while (true)
                {
                    if (regionLoadRequests.Count > 0)
                    {
                        List<Region.Position> loadedRegionPositions = new List<Region.Position>();
                        List<Region.Position> regionLoadRequestsCopy = new List<Region.Position>(regionLoadRequests);
                        foreach (Region.Position regionPosition in regionLoadRequestsCopy)
                        {
                            if (!currentUniverse.IsSectorLoaded(regionPosition.SectorPosition))
                            {
                                continue;
                            }

                            Sector sector = currentUniverse.GetSector(regionPosition.SectorPosition);
                            Region region = new Region(currentUniverse, regionPosition);
                            sector.RegisterRegionPosition(regionPosition);
                            currentUniverse.loadedRegions.Add(regionPosition, region);
                            currentUniverse.SaveSector(sector);
                            currentUniverse.SaveRegion(region);
                            loadedRegionPositions.Add(regionPosition);

                            yield return null;
                        }

                        foreach (Region.Position regionPosition in loadedRegionPositions)
                        {
                            regionLoadRequests.Remove(regionPosition);
                        }

                        yield return null;
                    }

                    yield return null;
                }
            }

            private IEnumerator ProcessRegionChunkLoadQueueCoroutine()
            {
                while (true)
                {
                    if (regionChunkLoadRequests.Count > 0)
                    {
                        List<Region.Chunk.Position> loadedRegionChunkPositions = new List<Region.Chunk.Position>();
                        List<Region.Chunk.Position> regionChunkLoadRequestsCopy = new List<Region.Chunk.Position>(regionChunkLoadRequests);
                        foreach (Region.Chunk.Position regionChunkPosition in regionChunkLoadRequestsCopy)
                        {
                            if (!currentUniverse.IsRegionLoaded(regionChunkPosition.RegionPosition))
                            {
                                yield return null;
                                continue;
                            }

                            Region region = currentUniverse.GetRegion(regionChunkPosition.RegionPosition);
                            Sector sector = currentUniverse.GetSector(regionChunkPosition.SectorPosition);
                            Region.Chunk regionChunk = new Region.Chunk(currentUniverse, sector, region, regionChunkPosition);
                            region.RegisterChunkPosition(regionChunkPosition);
                            currentUniverse.loadedRegionChunks.Add(regionChunkPosition, regionChunk);
                            currentUniverse.SaveRegion(region);
                            currentUniverse.SaveRegionChunk(regionChunk);
                            loadedRegionChunkPositions.Add(regionChunkPosition);

                            yield return null;
                        }

                        foreach (Region.Chunk.Position regionChunkPosition in loadedRegionChunkPositions)
                        {
                            regionChunkLoadRequests.Remove(regionChunkPosition);
                        }

                        yield return null;
                    }

                    yield return null;
                }
            }
            #endregion
        }

        // TODO: Add a Universe.Transform to this class maybe?
        [Serializable]
        public class Object : Component
        {
            #region Classes
            [Serializable]
            public class Transform
            {
                #region Classes
                [Serializable]
                public class Position
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

                #region Structs
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

                #region Classes
                [Serializable]
                public class Position
                {
                    #region Properties
                    public Vector2Int CurrentPosition => currentPosition;
                    public Vector2 WorldPosition
                    {
                        get
                        {
                            Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                            int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                            int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                            int filamentChunkSize = universe.generationSettings.FilamentGenerationSettings.ChunkSize;
                            Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                            return ((currentPosition.ToVector2() * regionSize) + regionOffset) * sectorSize * filamentChunkSize;
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
                    [SerializeField] private Vector2Int currentPosition;
                    #endregion

                    #region Constructors
                    public Position(Vector2Int chunkPosition)
                    {
                        this.currentPosition = chunkPosition;
                    }

                    public Position(Vector2 worldPosition)
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                        int filamentChunkSize = universe.generationSettings.FilamentGenerationSettings.ChunkSize;
                        currentPosition = (worldPosition / regionSize / sectorSize / filamentChunkSize).FloorToVector2Int();
                    }
                    #endregion

                    #region Overrides
                    public override bool Equals(object obj)
                    {
                        Position otherPosition = obj as Position;
                        return currentPosition.Equals(otherPosition.currentPosition);
                    }

                    public override int GetHashCode()
                    {
                        return currentPosition.GetHashCode();
                    }
                    #endregion
                }

                [Serializable]
                public class DensityMap
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
                    [SerializeField] public int Size;

                    public DensityMapCollection(int size)
                    {
                        ElectronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Electron);
                        PositronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Positron);
                        ProtonDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Proton);
                        AntiProtonDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.AntiProton);
                        NeutronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Neutron);
                        AntiNeutronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.AntiNeutron);
                        GenerationState = GenerationState.Generating;
                        Size = size;
                    }
                }
                #endregion

                #region Structs
                public struct DensityMapCoroutineInfo
                {
                    public readonly Action<DensityMapCollection> Callback;
                    public readonly DensityMapCollection DensityMaps;

                    public DensityMapCoroutineInfo(Action<DensityMapCollection> callback, DensityMapCollection densityMaps)
                    {
                        Callback = callback;
                        DensityMaps = densityMaps;
                    }
                }
                #endregion

                #region Static Properties
                public static Queue<DensityMapCoroutineInfo> DensityMapCoroutineInfoQueue => densityMapCoroutineInfoQueue;
                #endregion

                #region Static Fields
                private static Queue<DensityMapCoroutineInfo> densityMapCoroutineInfoQueue = new Queue<DensityMapCoroutineInfo>();
                #endregion

                #region Properties
                public int ChunkSeed => chunkSeed;
                public int Size => size;
                public Position FilamentChunkPosition => filamentChunkPosition;
                public DensityMapCollection DensityMaps => densityMaps;
                #endregion

                #region Fields
                [SerializeField] private int chunkSeed;
                [SerializeField] private int size;
                [SerializeField] private Position filamentChunkPosition;
                [SerializeField] private DensityMapCollection densityMaps;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Filament filament, Position chunkPosition)
                {
                    GenerationSettings filamentGenerationSettings = universe.FilamentGenerationSettings;
                    size = filamentGenerationSettings.ChunkSize;
                    this.filamentChunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + filament.filamentPosition.CurrentPosition.magnitude + chunkPosition.CurrentPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection(size);
                    RequestDensityMaps(universe, filament, OnDensityMapsReceived);
                }
                #endregion

                #region Methods
                private void RequestDensityMaps(Universe universe, Filament filament, Action<DensityMapCollection> callback)
                {
                    DensityMapGenerationUtil.Instance.StartCoroutine(DensityMapGenerationCoroutine(universe, filament, callback));
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

                private IEnumerator DensityMapGenerationCoroutine(Universe universe, Filament filament, Action<DensityMapCollection> callback)
                {
                    Filament.GenerationSettings filamentGenerationSettings = universe.FilamentGenerationSettings;
                    DensityMapCollection filamentDensityMaps = new DensityMapCollection(filamentGenerationSettings.ChunkSize);
                    DensityMapGenerationUtil.RequestGenerateFilamentDensityMaps(universe.UniverseDensityMap, filamentDensityMaps, (filamentDensityMaps) =>
                    {
                        lock (densityMapCoroutineInfoQueue)
                        {
                            densityMapCoroutineInfoQueue.Enqueue(new DensityMapCoroutineInfo(callback, filamentDensityMaps));
                        }
                    });

                    yield return null;
                }
                #endregion
            }

            [Serializable]
            public class Position
            {
                #region Properties
                public Vector2Int CurrentPosition => currentPosition;
                public Vector2 WorldPosition
                {
                    get
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                        int filamentSize = universe.generationSettings.FilamentGenerationSettings.Size;
                        Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                        return ((currentPosition * regionSize) + regionOffset) * sectorSize * filamentSize;
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
                [SerializeField] private Vector2Int currentPosition;
                #endregion

                #region Constructor
                public Position(Vector2Int chunkPosition)
                {
                    this.currentPosition = chunkPosition;
                }

                public Position(Vector2 worldPosition)
                {
                    Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    int filamentSize = universe.generationSettings.FilamentGenerationSettings.Size;
                    currentPosition = (worldPosition / regionSize / sectorSize / filamentSize).FloorToVector2Int();
                }
                #endregion

                #region Overrides
                public override bool Equals(object obj)
                {
                    Position otherPosition = obj as Position;
                    return currentPosition.Equals(otherPosition.currentPosition);
                }

                public override int GetHashCode()
                {
                    return currentPosition.GetHashCode();
                }
                #endregion
            }
            #endregion

            #region Structs
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
                public int Seed;
                public int ChunkSize;
                public int ChunkAmount;
                public float MapFromMin;
                public float MapFromMax;
                public float MapToMin;
                public float MapToMax;
                public float UniverseNoiseInfluence;
                public float Power;
                public float Frequency;
                public int Octaves;
                public float Persistence;
                public float Lacunarity;
                public float Amplitude;
                public float CellularJitter;
                #endregion
            }
            #endregion

            #region Properties
            public Position FilamentPosition => filamentPosition;
            public SerializableDictionary<Vector2Int, Chunk.Position> ChunkPositionMap => chunkPositionMap;
            public SerializableDictionary<Vector2Int, Sector.Position> SectorPositionMap => sectorPositionMap;
            #endregion

            #region Fields
            [SerializeField] private Position filamentPosition;
            [SerializeField] private SerializableDictionary<Vector2Int, Chunk.Position> chunkPositionMap;
            [SerializeField] private SerializableDictionary<Vector2Int, Sector.Position> sectorPositionMap;
            #endregion

            #region Constructors
            public Filament(Universe universe, Position filamentPosition)
            {
                this.filamentPosition = filamentPosition;
                chunkPositionMap = new SerializableDictionary<Vector2Int, Chunk.Position>();
                sectorPositionMap = new SerializableDictionary<Vector2Int, Sector.Position>();
            }
            #endregion

            #region Methods
            public void RegisterChunkPosition(Chunk.Position chunkPosition)
            {
                if (chunkPositionMap.ContainsKey(chunkPosition.CurrentPosition))
                {
                    throw new Exception("Chunk.Position is already registered!");
                }
                chunkPositionMap.Add(chunkPosition.CurrentPosition, chunkPosition);
            }

            public void UnregisterChunkPosition(Chunk.Position chunkPosition)
            {
                chunkPositionMap.Remove(chunkPosition.CurrentPosition);
            }

            public void RegisterSectorPosition(Sector.Position sectorPosition)
            {
                if (sectorPositionMap.ContainsKey(sectorPosition.CurrentPosition))
                {
                    throw new Exception("Sector.Position is already registered!");
                }
                sectorPositionMap.Add(sectorPosition.CurrentPosition, sectorPosition);
            }

            public void UnregisterSectorPosition(Sector.Position sectorPosition)
            {
                sectorPositionMap.Remove(sectorPosition.CurrentPosition);
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

                #region Classes
                [Serializable]
                public class Position
                {
                    #region Properties
                    public Vector2Int CurrentPosition => currentPosition;
                    public Vector2 WorldPosition
                    {
                        get
                        {
                            Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                            int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                            int sectorChunkSize = universe.generationSettings.SectorGenerationSettings.ChunkSize;
                            Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                            return ((currentPosition * regionSize) + regionOffset) * sectorChunkSize;
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
                    [SerializeField] private Vector2Int currentPosition;
                    #endregion

                    #region Constructors
                    public Position(Vector2Int chunkPosition)
                    {
                        this.currentPosition = chunkPosition;
                    }

                    public Position(Vector2 worldPosition)
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorChunkSize = universe.generationSettings.SectorGenerationSettings.ChunkSize;
                        currentPosition = (worldPosition / regionSize / sectorChunkSize).FloorToVector2Int();
                    }
                    #endregion

                    #region Overrides
                    public override bool Equals(object obj)
                    {
                        Position otherPosition = obj as Position;
                        return currentPosition.Equals(otherPosition.currentPosition);
                    }

                    public override int GetHashCode()
                    {
                        return currentPosition.GetHashCode();
                    }
                    #endregion
                }

                [Serializable]
                public class DensityMap
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

                [Serializable]
                public class DensityMapCollection
                {
                    [SerializeField] public DensityMap SolidParticleDensityMap;
                    [SerializeField] public DensityMap LiquidParticleDensityMap;
                    [SerializeField] public DensityMap GasParticleDensityMap;
                    [SerializeField] public DensityMap PlasmaParticleDensityMap;
                    [SerializeField] public GenerationState GenerationState;
                    [SerializeField] public int Size;

                    public DensityMapCollection(int size)
                    {
                        SolidParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.SolidParticle);
                        LiquidParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.LiquidParticle);
                        GasParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.GasParticle);
                        PlasmaParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.PlasmaParticle);
                        GenerationState = GenerationState.Generating;
                        Size = size;
                    }
                }
                #endregion

                #region Structs
                public struct DensityMapCoroutineInfo
                {
                    public readonly Action<DensityMapCollection> Callback;
                    public readonly DensityMapCollection DensityMaps;

                    public DensityMapCoroutineInfo(Action<DensityMapCollection> callback, DensityMapCollection densityMaps)
                    {
                        Callback = callback;
                        DensityMaps = densityMaps;
                    }
                }
                #endregion

                #region Static Properties
                public static Queue<DensityMapCoroutineInfo> DensityMapCoroutineInfoQueue => densityMapCoroutineInfoQueue;
                #endregion

                #region Static Fields
                private static Queue<DensityMapCoroutineInfo> densityMapCoroutineInfoQueue = new Queue<DensityMapCoroutineInfo>();
                #endregion

                #region Properties
                public int ChunkSeed => chunkSeed;
                public int Size => size;
                public Position SectorChunkPosition => sectorChunkPosition;
                public DensityMapCollection DensityMaps => densityMaps;
                #endregion

                #region Fields
                [SerializeField] private int chunkSeed;
                [SerializeField] private int size;
                [SerializeField] private Position sectorChunkPosition;
                [SerializeField] private DensityMapCollection densityMaps;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Filament filament, Sector sector, Position chunkPosition)
                {
                    GenerationSettings sectorGenerationSettings = universe.SectorGenerationSettings;
                    size = sectorGenerationSettings.ChunkSize;
                    this.sectorChunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + sector.sectorPosition.CurrentPosition.magnitude + chunkPosition.CurrentPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection(size);
                    RequestDensityMaps(universe, filament, sector, OnDensityMapsReceived);
                }
                #endregion
                
                #region Methods
                private void RequestDensityMaps(Universe universe, Filament filament, Sector sector, Action<DensityMapCollection> callback)
                {
                    DensityMapGenerationUtil.Instance.StartCoroutine(DensityMapGenerationCoroutine(universe, filament, sector, callback));
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

                private IEnumerator DensityMapGenerationCoroutine(Universe universe, Filament filament, Sector sector, Action<DensityMapCollection> callback)
                {
                    Filament.GenerationSettings filamentGenerationSettings = universe.FilamentGenerationSettings;
                    Sector.GenerationSettings sectorGenerationSettings = universe.SectorGenerationSettings;
                    Filament.Chunk.DensityMapCollection filamentDensityMaps = new Filament.Chunk.DensityMapCollection(filamentGenerationSettings.ChunkSize);
                    DensityMapCollection sectorDensityMaps = new DensityMapCollection(sectorGenerationSettings.ChunkSize);
                    DensityMapGenerationUtil.RequestGenerateSectorDensityMaps(filamentDensityMaps, sectorDensityMaps, (sectorDensityMaps) =>
                    {
                        lock (densityMapCoroutineInfoQueue)
                        {
                            densityMapCoroutineInfoQueue.Enqueue(new DensityMapCoroutineInfo(callback, sectorDensityMaps));
                        }
                    });

                    yield return null;
                }
                #endregion
            }

            [Serializable]
            public class Position
            {
                #region Properties
                public Vector2Int CurrentPosition => currentPosition;
                public Vector2 WorldPosition
                {
                    get
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                        Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                        return ((currentPosition * regionSize) + regionOffset) * sectorSize;
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
                [SerializeField] private Vector2Int currentPosition;
                #endregion

                #region Constructors
                public Position(Vector2Int chunkPosition)
                {
                    this.currentPosition = chunkPosition;
                }

                public Position(Vector2 worldPosition)
                {
                    Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    int sectorSize = universe.generationSettings.SectorGenerationSettings.Size;
                    currentPosition = (worldPosition / regionSize / sectorSize).FloorToVector2Int();
                }
                #endregion

                #region Overrides
                public override bool Equals(object obj)
                {
                    Position otherPosition = obj as Position;
                    return currentPosition.Equals(otherPosition.currentPosition);
                }

                public override int GetHashCode()
                {
                    return currentPosition.GetHashCode();
                }
                #endregion
            }
            #endregion

            #region Structs
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
                public int Seed;
                public int ChunkSize;
                public int ChunkAmount;
                public float MapFromMin;
                public float MapFromMax;
                public float MapToMin;
                public float MapToMax;
                public float FilamentNoiseInfluence;
                public float Power;
                public float Frequency;
                public int Octaves;
                public float Persistence;
                public float Lacunarity;
                public float Amplitude;
                #endregion
            }
            #endregion

            #region Properties
            public Position SectorPosition => sectorPosition;
            public SerializableDictionary<Vector2Int, Chunk.Position> ChunkPositionMap => chunkPositionMap;
            public SerializableDictionary<Vector2Int, Region.Position> RegionPositionMap => regionPositionMap;
            #endregion

            #region Fields
            [SerializeField] private Position sectorPosition;
            [SerializeField] private SerializableDictionary<Vector2Int, Chunk.Position> chunkPositionMap;
            [SerializeField] private SerializableDictionary<Vector2Int, Region.Position> regionPositionMap;
            #endregion

            #region Constructors
            public Sector(Universe universe, Position sectorPosition)
            {
                this.sectorPosition = sectorPosition;
                chunkPositionMap = new SerializableDictionary<Vector2Int, Chunk.Position>();
                regionPositionMap = new SerializableDictionary<Vector2Int, Region.Position>();
            }
            #endregion

            #region Methods
            public void RegisterChunkPosition(Chunk.Position chunkPosition)
            {
                if (chunkPositionMap.ContainsKey(chunkPosition.CurrentPosition))
                {
                    throw new Exception("Chunk.Position is already registered!");
                }
                chunkPositionMap.Add(chunkPosition.CurrentPosition, chunkPosition);
            }

            public void UnregisterChunkPosition(Chunk.Position chunkPosition)
            {
                chunkPositionMap.Remove(chunkPosition.CurrentPosition);
            }

            public void RegisterRegionPosition(Region.Position regionPosition)
            {
                if (regionPositionMap.ContainsKey(regionPosition.CurrentPosition))
                {
                    throw new Exception("Region.Position is already registered!");
                }
                regionPositionMap.Add(regionPosition.CurrentPosition, regionPosition);
            }

            public void UnregisterRegionPosition(Region.Position regionPosition)
            {
                regionPositionMap.Remove(regionPosition.CurrentPosition);
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

                #region Classes
                [Serializable]
                public class Position
                {
                    #region Properties
                    public Vector2Int CurrentPosition => currentPosition;
                    public Vector2 WorldPosition
                    {
                        get
                        {
                            Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                            int regionChunkSize = universe.generationSettings.RegionGenerationSettings.ChunkSize;
                            Vector2 regionChunkOffset = new Vector2(regionChunkSize / 2.0f, regionChunkSize / 2.0f);
                            return (currentPosition.ToVector2() * regionChunkSize) + regionChunkOffset;
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
                    [SerializeField] private Vector2Int currentPosition;
                    #endregion

                    #region Constructors
                    public Position(Vector2Int chunkPosition)
                    {
                        this.currentPosition = chunkPosition;
                    }

                    public Position(Vector2 worldPosition)
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionChunkSize = universe.generationSettings.RegionGenerationSettings.ChunkSize;
                        currentPosition = (worldPosition / regionChunkSize).FloorToVector2Int();
                    }
                    #endregion

                    #region Overrides
                    public override bool Equals(object obj)
                    {
                        Position otherPosition = obj as Position;
                        return currentPosition.Equals(otherPosition.currentPosition);
                    }

                    public override int GetHashCode()
                    {
                        return currentPosition.GetHashCode();
                    }
                    #endregion
                }

                [Serializable]
                public class DensityMap
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

                [Serializable]
                public class DensityMapCollection
                {
                    [SerializeField] public DensityMap MatterDensityMap;
                    [SerializeField] public DensityMap AntiMatterDensityMap;
                    [SerializeField] public GenerationState GenerationState;
                    [SerializeField] public int Size;

                    public DensityMapCollection(int size)
                    {
                        MatterDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Matter);
                        AntiMatterDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.AntiMatter);
                        GenerationState = GenerationState.Generating;
                        Size = size;
                    }
                }
                #endregion

                #region Structs
                public struct DensityMapCoroutineInfo
                {
                    public readonly Action<DensityMapCollection> Callback;
                    public readonly DensityMapCollection DensityMaps;

                    public DensityMapCoroutineInfo(Action<DensityMapCollection> callback, DensityMapCollection densityMaps)
                    {
                        Callback = callback;
                        DensityMaps = densityMaps;
                    }
                }
                #endregion

                #region Static Properties
                public static Queue<DensityMapCoroutineInfo> DensityMapCoroutineInfoQueue => densityMapCoroutineInfoQueue;
                #endregion

                #region Static Fields
                private static Queue<DensityMapCoroutineInfo> densityMapCoroutineInfoQueue = new Queue<DensityMapCoroutineInfo>();
                #endregion

                #region Properties
                public int ChunkSeed => chunkSeed;
                public int Size => size;
                public Position RegionChunkPosition => regionChunkPosition;
                public DensityMapCollection DensityMaps => densityMaps;
                #endregion

                #region Fields
                [SerializeField] private int chunkSeed;
                [SerializeField] private int size;
                [SerializeField] private Position regionChunkPosition;
                [SerializeField] private DensityMapCollection densityMaps;
                #endregion

                #region Constructors
                public Chunk(Universe universe, Sector sector, Region region, Position chunkPosition)
                {
                    GenerationSettings regionGenerationSettings = universe.RegionGenerationSettings;
                    size = regionGenerationSettings.ChunkSize;
                    this.regionChunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + region.regionPosition.CurrentPosition.magnitude + chunkPosition.CurrentPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection(regionGenerationSettings.ChunkSize);
                    RequestDensityMaps(universe, sector, region, OnDensityMapsReceived);
                }
                #endregion

                #region Methods
                private void RequestDensityMaps(Universe universe, Sector sector, Region region, Action<DensityMapCollection> callback)
                {
                    DensityMapGenerationUtil.Instance.StartCoroutine(DensityMapGenerationCoroutine(universe, sector, region, callback));
                }

                private void OnDensityMapsReceived(DensityMapCollection densityMaps)
                {
                    this.densityMaps.MatterDensityMap = densityMaps.MatterDensityMap;
                    this.densityMaps.AntiMatterDensityMap = densityMaps.AntiMatterDensityMap;
                    this.densityMaps.GenerationState = densityMaps.GenerationState;
                    GameManager.Instance.CurrentGame.CurrentUniverse.SaveRegionChunk(this);
                }

                private IEnumerator DensityMapGenerationCoroutine(Universe universe, Sector sector, Region region, Action<DensityMapCollection> callback)
                {
                    Sector.GenerationSettings sectorGenerationSettings = universe.SectorGenerationSettings;
                    Region.GenerationSettings regionGenerationSettings = universe.RegionGenerationSettings;
                    Sector.Chunk.DensityMapCollection sectorDensityMaps = new Sector.Chunk.DensityMapCollection(sectorGenerationSettings.ChunkSize);
                    DensityMapCollection regionDensityMaps = new DensityMapCollection(regionGenerationSettings.ChunkSize);
                    DensityMapGenerationUtil.RequestRegionDensityMapsGeneration(sectorDensityMaps, regionDensityMaps, (regionDensityMaps) =>
                    {
                        lock (densityMapCoroutineInfoQueue)
                        {
                            densityMapCoroutineInfoQueue.Enqueue(new DensityMapCoroutineInfo(callback, regionDensityMaps));
                        }
                    });

                    yield return null;
                }
                #endregion
            }
            
            [Serializable]
            public class Position
            {
                #region Properties
                public Vector2Int CurrentPosition => currentPosition;
                public Vector2 WorldPosition
                {
                    get
                    {
                        Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                        int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                        Vector2 regionOffset = new Vector2(regionSize / 2.0f, regionSize / 2.0f);
                        return (currentPosition * regionSize) + regionOffset;
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
                [SerializeField] private Vector2Int currentPosition;
                #endregion

                #region Constructors
                public Position(Vector2Int chunkPosition)
                {
                    this.currentPosition = chunkPosition;
                }

                public Position(Vector2 worldPosition)
                {
                    Universe universe = GameManager.Instance.CurrentGame.CurrentUniverse;
                    int regionSize = universe.generationSettings.RegionGenerationSettings.Size;
                    currentPosition = (worldPosition / regionSize).FloorToVector2Int();
                }
                #endregion

                #region Overrides
                public override bool Equals(object obj)
                {
                    Position otherPosition = obj as Position;
                    return currentPosition.Equals(otherPosition.currentPosition);
                }

                public override int GetHashCode()
                {
                    return currentPosition.GetHashCode();
                }
                #endregion
            }
            #endregion

            #region Structs
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
                public int Seed;
                public int ChunkSize;
                public int ChunkAmount;
                public float MapFromMin;
                public float MapFromMax;
                public float MapToMin;
                public float MapToMax;
                public float SectorNoiseInfluence;
                public float Power;
                public float Frequency;
                public int Octaves;
                public float Persistence;
                public float Lacunarity;
                public float Amplitude;
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
            public void RegisterChunkPosition(Chunk.Position chunkPosition)
            {
                if (chunkPositionMap.ContainsKey(chunkPosition.CurrentPosition))
                {
                    throw new Exception("Chunk.Position is already registered!");
                }
                chunkPositionMap.Add(chunkPosition.CurrentPosition, chunkPosition);
            }

            public void UnregisterChunkPosition(Chunk.Position chunkPosition)
            {
                chunkPositionMap.Remove(chunkPosition.CurrentPosition);
            }
            #endregion
        }
        #endregion

        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            #region Fields
            public int Seed;
            public int Size;
            public float MapFromMin;
            public float MapFromMax;
            public float MapToMin;
            public float MapToMax;
            public float UniverseNoiseInfluence;
            public float Power;
            public float Frequency;
            public int Octaves;
            public float Persistence;
            public float Lacunarity;
            public float Amplitude;
            public float CellularJitter;

            public Filament.GenerationSettings FilamentGenerationSettings;
            public Sector.GenerationSettings SectorGenerationSettings;
            public Region.GenerationSettings RegionGenerationSettings;
            #endregion
        }

        [Serializable]
        public struct DensityMap
        {
            public SerializableDictionary<Vector2Int, float> DensityMapDictionary => densityMapDictionary;

            [SerializeField] private SerializableDictionary<Vector2Int, float> densityMapDictionary;

            public DensityMap(SerializableDictionary<Vector2Int, float> densityMapDictionary)
            {
                this.densityMapDictionary = densityMapDictionary;
            }
        }
        #endregion

        #region Static Properties
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
                generationSettings.Frequency = 0.04f;
                generationSettings.Octaves = 3;
                generationSettings.Persistence = 0.5f;
                generationSettings.Lacunarity = 2.0f;
                generationSettings.Amplitude = 64.0f;
                generationSettings.CellularJitter = 1.0f;
                #endregion

                #region Filament Generation Settings Default
                generationSettings.FilamentGenerationSettings.Seed = generationSettings.Seed;
                generationSettings.FilamentGenerationSettings.ChunkSize = 64;
                generationSettings.FilamentGenerationSettings.ChunkAmount = 16;
                generationSettings.FilamentGenerationSettings.MapFromMin = -1.0f;
                generationSettings.FilamentGenerationSettings.MapFromMax = 1.0f;
                generationSettings.FilamentGenerationSettings.MapToMin = 0.0f;
                generationSettings.FilamentGenerationSettings.MapToMax = 1.0f;
                generationSettings.FilamentGenerationSettings.UniverseNoiseInfluence = 1.0f;
                generationSettings.FilamentGenerationSettings.Power = 1.0f;
                generationSettings.FilamentGenerationSettings.Frequency = 0.02f;
                generationSettings.FilamentGenerationSettings.Octaves = 5;
                generationSettings.FilamentGenerationSettings.Persistence = 0.5f;
                generationSettings.FilamentGenerationSettings.Lacunarity = 2.0f;
                generationSettings.FilamentGenerationSettings.Amplitude = 1.0f;
                generationSettings.FilamentGenerationSettings.CellularJitter = 1.0f;
                #endregion

                #region Sector Generation Settings Default
                generationSettings.SectorGenerationSettings.ChunkSize = 64;
                generationSettings.SectorGenerationSettings.ChunkAmount = 16;
                generationSettings.SectorGenerationSettings.MapFromMin = -1.0f;
                generationSettings.SectorGenerationSettings.MapFromMax = 1.0f;
                generationSettings.SectorGenerationSettings.MapToMin = 0.0f;
                generationSettings.SectorGenerationSettings.MapToMax = 1.0f;
                generationSettings.SectorGenerationSettings.FilamentNoiseInfluence = 1.0f;
                generationSettings.SectorGenerationSettings.Power = 1.0f;
                generationSettings.SectorGenerationSettings.Frequency = 0.01f;
                generationSettings.SectorGenerationSettings.Octaves = 5;
                generationSettings.SectorGenerationSettings.Persistence = 0.5f;
                generationSettings.SectorGenerationSettings.Lacunarity = 2.0f;
                generationSettings.SectorGenerationSettings.Amplitude = 1.0f;
                #endregion

                #region Region Generation Settings Default
                generationSettings.RegionGenerationSettings.ChunkSize = 64;
                generationSettings.RegionGenerationSettings.ChunkAmount = 16;
                generationSettings.RegionGenerationSettings.MapFromMin = -1.0f;
                generationSettings.RegionGenerationSettings.MapFromMax = 1.0f;
                generationSettings.RegionGenerationSettings.MapToMin = -0.375f;
                generationSettings.RegionGenerationSettings.MapToMax = 1.375f;
                generationSettings.RegionGenerationSettings.SectorNoiseInfluence = 1.0f;
                generationSettings.RegionGenerationSettings.Power = 1.0f;
                generationSettings.SectorGenerationSettings.Frequency = 0.005f;
                generationSettings.SectorGenerationSettings.Octaves = 5;
                generationSettings.SectorGenerationSettings.Persistence = 0.5f;
                generationSettings.SectorGenerationSettings.Lacunarity = 2.0f;
                generationSettings.RegionGenerationSettings.Amplitude = 1.0f;
                #endregion

                return generationSettings;
            }
        }
        #endregion

>>>>>>> develop:Assets/Mods/LooCast/Modules/Universe/Assets/Scripts/Universe.cs
        #region Properties
        public UniverseUnityComponent UniverseUnityComponent { get; private set; }
        #endregion

        #region Fields
        public int chunkSize;
        private Dictionary<int, Scale> scaleDictionary;
        #endregion

        #region Constructors
        public Universe(int chunkSize = 32) : base()
        {
            this.chunkSize = chunkSize;
            
            scaleDictionary = new Dictionary<int, Scale>();

            EnableUnityBridge();
        }
        #endregion

        #region Static Methods
        #endregion

        #region Methods
        [LuaMethod("GetChunkSize")]
        public int GetChunkSize()
        {
            return chunkSize;
        }

        public bool IsScaleGenerated(int scaleLevel)
        {
            return scaleDictionary.ContainsKey(scaleLevel);
        }

        public void GenerateScale(int scaleLevel)
        {
            if (scaleDictionary.ContainsKey(scaleLevel))
            {
                throw new Exception($"Scale '{scaleLevel}' has already been generated!");
            }

            Scale scale = new Scale(scaleLevel, this);
            scaleDictionary.Add(scaleLevel, scale);
        }
        
        public Scale GetScale(int scaleLevel)
        {
            if (!scaleDictionary.TryGetValue(scaleLevel, out Scale scale))
            {
                throw new Exception($"Scale '{scaleLevel}' has not been generated!");
            }
            
            return scale;
        }

        public void DeleteScale(int scaleLevel)
        {
            if (!scaleDictionary.ContainsKey(scaleLevel))
            {
                throw new Exception($"Scale '{scaleLevel}' has already been deleted!");
            }

            scaleDictionary[scaleLevel].DisableUnityBridge();
            scaleDictionary.Remove(scaleLevel);
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();

            UnityBridge.RootGameObject.name = ToString();

            UniverseUnityComponent = UnityBridge.RootGameObject.AddComponent<UniverseUnityComponent>();
            UniverseUnityComponent.Setup(this);
        }

        public override void DisableUnityBridge()
        {
            base.DisableUnityBridge();

            UniverseUnityComponent = null;
        }

        public override string ToString()
        {
            return $"Universe";
        }
        #endregion
    }
}