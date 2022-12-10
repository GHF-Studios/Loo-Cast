using UnityEngine;
using System.Collections;
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

    [Serializable]
    public class Universe
    {
        #region Classes
        [Serializable]
        public class ParallelizationUtil : MonoBehaviour
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

            private struct FilamentGenerationSettingsGPU
            {
                public readonly int ChunkSize;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float UniverseNoiseInfluence;
                public readonly float Power;
                public readonly float Amplitude;

                public FilamentGenerationSettingsGPU(Universe.Filament.GenerationSettings settings)
                {
                    ChunkSize = settings.ChunkSize;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    UniverseNoiseInfluence = settings.UniverseNoiseInfluence;
                    Power = settings.Power;
                    Amplitude = settings.Amplitude;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) + sizeof(float) * 7;
                    }
                }
            }

            private struct SectorGenerationSettingsGPU
            {
                public readonly int ChunkSize;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float FilamentNoiseInfluence;
                public readonly float Power;
                public readonly float Amplitude;

                public SectorGenerationSettingsGPU(Universe.Sector.GenerationSettings settings)
                {
                    ChunkSize = settings.ChunkSize;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    FilamentNoiseInfluence = settings.FilamentNoiseInfluence;
                    Power = settings.Power;
                    Amplitude = settings.Amplitude;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) + sizeof(float) * 7;
                    }
                }
            }

            private struct RegionGenerationSettingsGPU
            {
                public readonly int ChunkSize;
                public readonly float MapFromMin;
                public readonly float MapFromMax;
                public readonly float MapToMin;
                public readonly float MapToMax;
                public readonly float SectorNoiseInfluence;
                public readonly float Power;
                public readonly float Amplitude;

                public RegionGenerationSettingsGPU(Universe.Region.GenerationSettings settings)
                {
                    ChunkSize = settings.ChunkSize;
                    MapFromMin = settings.MapFromMin;
                    MapFromMax = settings.MapFromMax;
                    MapToMin = settings.MapToMin;
                    MapToMax = settings.MapToMax;
                    SectorNoiseInfluence = settings.SectorNoiseInfluence;
                    Power = settings.Power;
                    Amplitude = settings.Amplitude;
                }

                public static int ByteSize
                {
                    get
                    {
                        return sizeof(int) + sizeof(float) * 7;
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
            public static ParallelizationUtil Instance => instance;
            #endregion

            #region Static Fields
            private static ParallelizationUtil instance;
            private static Queue<FilamentDensityMapCoroutineInfo> filamentDensityMapCoroutineInfoQueue = new Queue<FilamentDensityMapCoroutineInfo>();
            private static Queue<SectorDensityMapCoroutineInfo> sectorDensityMapCoroutineInfoQueue = new Queue<SectorDensityMapCoroutineInfo>();
            private static Queue<RegionDensityMapCoroutineInfo> regionDensityMapCoroutineInfoQueue = new Queue<RegionDensityMapCoroutineInfo>();
            #endregion

            #region Fields
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
                if (instance != null)
                {
                    throw new Exception("Cannot have multiple instances of Universe.ParallelizationUtil!");
                }
                GameObject instanceObject = new GameObject("[Universe.ParallelizationUtil]");
                instanceObject.layer = 31;
                instanceObject.tag = "INTERNAL";
                instance = instanceObject.AddComponent<ParallelizationUtil>();
                DontDestroyOnLoad(instance);

                instance.filamentDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/FilamentDensityComputeShader");
                instance.sectorDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/SectorDensityComputeShader");
                instance.regionDensityShader = Resources.Load<ComputeShader>("Shaders/Computation/Universe/RegionDensityComputeShader");

                Debug.Log("[Universe.ParallelizationUtil] Initialized.");
            }
            
            public static void RequestProcessedFilamentDensityMaps(DensityMap universeDensityMap, Filament.Chunk.DensityMapCollection filamentDensityMaps, Action<Filament.Chunk.DensityMapCollection> callback)
            {
                if (instance == null)
                {
                    throw new Exception("Universe.ParallelizationUtil has not been initialized!");
                }
                
                instance.StartCoroutine(instance.FilamentDensityMapsProcessingCoroutine(universeDensityMap, filamentDensityMaps, callback));
            }

            public static void RequestProcessedSectorDensityMaps(Filament.Chunk.DensityMapCollection filamentDensityMaps, Sector.Chunk.DensityMapCollection sectorDensityMaps, Action<Sector.Chunk.DensityMapCollection> callback)
            {
                if (instance == null)
                {
                    throw new Exception("Universe.ParallelizationUtil has not been initialized!");
                }

                instance.StartCoroutine(instance.SectorDensityMapsProcessingCoroutine(filamentDensityMaps, sectorDensityMaps, callback));
            }

            public static void RequestProcessedRegionDensityMaps(Sector.Chunk.DensityMapCollection sectorDensityMaps, Region.Chunk.DensityMapCollection regionDensityMaps, Action<Region.Chunk.DensityMapCollection> callback)
            {
                if (instance == null)
                {
                    throw new Exception("Universe.ParallelizationUtil has not been initialized!");
                }

                instance.StartCoroutine(instance.RegionDensityMapsProcessingCoroutine(sectorDensityMaps, regionDensityMaps, callback));
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
            private IEnumerator FilamentDensityMapsProcessingCoroutine(DensityMap universeDensityMap, Filament.Chunk.DensityMapCollection filamentDensityMaps, Action<Filament.Chunk.DensityMapCollection> callback)
            {
                GenerationSettings universeGenerationSettings = GameManager.Instance.CurrentGame.CurrentUniverse.UniverseGenerationSettings;
                Filament.GenerationSettings filamentGenerationSettings = universeGenerationSettings.FilamentGenerationSettings;
                FilamentGenerationSettingsGPU[] filamentGenerationSettingsData = { new FilamentGenerationSettingsGPU(filamentGenerationSettings) };
                DensityDataGPU[] universeDensitiesData = new DensityDataGPU[universeDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] electronDensitiesData = new DensityDataGPU[filamentDensityMaps.ElectronDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] positronDensitiesData = new DensityDataGPU[filamentDensityMaps.PositronDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] protonDensitiesData = new DensityDataGPU[filamentDensityMaps.ProtonDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] antiProtonDensitiesData = new DensityDataGPU[filamentDensityMaps.AntiProtonDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] neutronDensitiesData = new DensityDataGPU[filamentDensityMaps.NeutronDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] antiNeutronDensitiesData = new DensityDataGPU[filamentDensityMaps.AntiNeutronDensityMap.DensityMapDictionary.EntryArray.Length];

                for (int x = 0; x < filamentGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < filamentGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * filamentGenerationSettings.ChunkSize + y;
                        universeDensitiesData[index] = new DensityDataGPU(x, y, universeDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        electronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.ElectronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        positronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.PositronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        protonDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.ProtonDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        antiProtonDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.AntiProtonDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        neutronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.NeutronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        antiNeutronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.AntiNeutronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
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

            private IEnumerator SectorDensityMapsProcessingCoroutine(Filament.Chunk.DensityMapCollection filamentDensityMaps, Sector.Chunk.DensityMapCollection sectorDensityMaps, Action<Sector.Chunk.DensityMapCollection> callback)
            {
                GenerationSettings universeGenerationSettings = GameManager.Instance.CurrentGame.CurrentUniverse.UniverseGenerationSettings;
                Sector.GenerationSettings sectorGenerationSettings = universeGenerationSettings.SectorGenerationSettings;
                SectorGenerationSettingsGPU[] sectorGenerationSettingsData = { new SectorGenerationSettingsGPU(sectorGenerationSettings) };
                DensityDataGPU[] solidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.SolidParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] liquidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.LiquidParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] gasParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.GasParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] plasmaParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.PlasmaParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] electronDensitiesData = new DensityDataGPU[filamentDensityMaps.ElectronDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] positronDensitiesData = new DensityDataGPU[filamentDensityMaps.PositronDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] protonDensitiesData = new DensityDataGPU[filamentDensityMaps.ProtonDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] antiProtonDensitiesData = new DensityDataGPU[filamentDensityMaps.AntiProtonDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] neutronDensitiesData = new DensityDataGPU[filamentDensityMaps.NeutronDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] antiNeutronDensitiesData = new DensityDataGPU[filamentDensityMaps.AntiNeutronDensityMap.DensityMapDictionary.EntryArray.Length];

                for (int x = 0; x < sectorGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < sectorGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * sectorGenerationSettings.ChunkSize + y;
                        solidParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.SolidParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        liquidParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.LiquidParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        gasParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.GasParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        plasmaParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.PlasmaParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        electronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.ElectronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        positronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.PositronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        protonDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.ProtonDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        antiProtonDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.AntiProtonDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        neutronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.NeutronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        antiNeutronDensitiesData[index] = new DensityDataGPU(x, y, filamentDensityMaps.AntiNeutronDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
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
            
            private IEnumerator RegionDensityMapsProcessingCoroutine(Sector.Chunk.DensityMapCollection sectorDensityMaps, Region.Chunk.DensityMapCollection regionDensityMaps, Action<Region.Chunk.DensityMapCollection> callback)
            {
                GenerationSettings universeGenerationSettings = GameManager.Instance.CurrentGame.CurrentUniverse.UniverseGenerationSettings;
                Region.GenerationSettings regionGenerationSettings = universeGenerationSettings.RegionGenerationSettings;
                RegionGenerationSettingsGPU[] regionGenerationSettingsData = { new RegionGenerationSettingsGPU(regionGenerationSettings) };
                DensityDataGPU[] matterParticleDensitiesData = new DensityDataGPU[regionDensityMaps.MatterDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] antiMatterParticleDensitiesData = new DensityDataGPU[regionDensityMaps.AntiMatterDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] solidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.SolidParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] liquidParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.LiquidParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] gasParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.GasParticleDensityMap.DensityMapDictionary.EntryArray.Length];
                DensityDataGPU[] plasmaParticleDensitiesData = new DensityDataGPU[sectorDensityMaps.PlasmaParticleDensityMap.DensityMapDictionary.EntryArray.Length];

                for (int x = 0; x < regionGenerationSettings.ChunkSize; x++)
                {
                    for (int y = 0; y < regionGenerationSettings.ChunkSize; y++)
                    {
                        int index = x * regionGenerationSettings.ChunkSize + y;
                        matterParticleDensitiesData[index] = new DensityDataGPU(x, y, regionDensityMaps.MatterDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        antiMatterParticleDensitiesData[index] = new DensityDataGPU(x, y, regionDensityMaps.AntiMatterDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        solidParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.SolidParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        liquidParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.LiquidParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        gasParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.GasParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
                        plasmaParticleDensitiesData[index] = new DensityDataGPU(x, y, sectorDensityMaps.PlasmaParticleDensityMap.DensityMapDictionary.GetEntry(new Vector2Int(x, y)).Value);
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
            #endregion
        }

        // TODO: Add a Universe.Transform to this class maybe?
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
                
                private struct DensityMapCoroutineInfo
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

                #region Classes
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

                    public DensityMapCollection()
                    {
                        ElectronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Electron);
                        PositronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Positron);
                        ProtonDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Proton);
                        AntiProtonDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.AntiProton);
                        NeutronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Neutron);
                        AntiNeutronDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.AntiNeutron);
                        GenerationState = GenerationState.Generating;
                    }
                }
                #endregion

                #region Static Fields
                private static Queue<DensityMapCoroutineInfo> densityMapCoroutineInfoQueue = new Queue<DensityMapCoroutineInfo>();
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
                    GenerationSettings filamentGenerationSettings = universe.FilamentGenerationSettings;
                    size = filamentGenerationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + filament.filamentPosition.ChunkPosition.magnitude + chunkPosition.ChunkPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection();
                    RequestDensityMaps(universe, filament, OnDensityMapsReceived);
                }
                #endregion

                #region Static Methods
                public static void ProcessDensityMapThreadInfoQueue()
                {
                    while (densityMapCoroutineInfoQueue.Count > 0)
                    {
                        DensityMapCoroutineInfo threadInfo = densityMapCoroutineInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);
                    }
                }
                #endregion

                #region Methods
                private void SampleDensityMaps(Universe universe, Filament filament, out Universe.DensityMap universeDensityMap, out DensityMapCollection filamentDensityMaps)
                {
                    GenerationSettings filamentGenerationSettings = universe.FilamentGenerationSettings;
                    universeDensityMap = new Universe.DensityMap(new SerializableDictionary<Vector2Int, float>());
                    filamentDensityMaps = new DensityMapCollection();

                    for (int y = 0; y < filamentGenerationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < filamentGenerationSettings.ChunkSize; x++)
                        {
                            #region Universe Noise Sampling
                            float universeOffsetX = -(1 / filamentGenerationSettings.ChunkAmount / filamentGenerationSettings.ChunkSize * x);
                            float universeOffsetY = -(1 / filamentGenerationSettings.ChunkAmount / filamentGenerationSettings.ChunkSize * y);

                            float universeSampleX = filament.FilamentPosition.ChunkPosition.x + universeOffsetX;
                            float universeSampleY = filament.FilamentPosition.ChunkPosition.y + universeOffsetY;

                            float universeDensity = universe.SampleNoise(universeSampleX, universeSampleY);
                            
                            universeDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), universeDensity);
                            #endregion
                            
                            #region Filament Noise Sampling
                            float filamentOffsetX = -((filament.FilamentPosition.ChunkPosition.x * filamentGenerationSettings.Size) + chunkPosition.ChunkPosition.x * filamentGenerationSettings.ChunkSize);
                            float filamentOffsetY = -((filament.FilamentPosition.ChunkPosition.y * filamentGenerationSettings.Size) + chunkPosition.ChunkPosition.y * filamentGenerationSettings.ChunkSize);

                            float filamentSampleX = x + filamentOffsetX;
                            float filamentSampleY = y + filamentOffsetY;

                            float electronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float positronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float protonDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float antiProtonDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float neutronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float antiNeutronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            
                            filamentDensityMaps.ElectronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), electronDensity);
                            filamentDensityMaps.PositronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), positronDensity);
                            filamentDensityMaps.ProtonDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), protonDensity);
                            filamentDensityMaps.AntiProtonDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), antiProtonDensity);
                            filamentDensityMaps.NeutronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), neutronDensity);
                            filamentDensityMaps.AntiNeutronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), antiNeutronDensity);
                            #endregion
                        }
                    }
                }
                
                private void RequestDensityMaps(Universe universe, Filament filament, Action<DensityMapCollection> callback)
                {
                    ParallelizationUtil.Instance.StartCoroutine(DensityMapGenerationCoroutine(universe, filament, callback));
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
                    SampleDensityMaps(universe, filament, out Universe.DensityMap universeDensityMap, out DensityMapCollection filamentDensityMaps);
                    ParallelizationUtil.RequestProcessedFilamentDensityMaps(universeDensityMap, filamentDensityMaps, (filamentDensityMaps) =>
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
                universe.FilamentDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                return universe.FilamentNoiseGenerator.GetNoise(sampleX, sampleY);
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

                private struct DensityMapCoroutineInfo
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

                #region Classes
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

                    public DensityMapCollection()
                    {
                        SolidParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.SolidParticle);
                        LiquidParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.LiquidParticle);
                        GasParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.GasParticle);
                        PlasmaParticleDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.PlasmaParticle);
                        GenerationState = GenerationState.Generating;
                    }
                }
                #endregion

                #region Static Fields
                private static Queue<DensityMapCoroutineInfo> densityMapCoroutineInfoQueue = new Queue<DensityMapCoroutineInfo>();
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
                public Chunk(Universe universe, Filament filament, Sector sector, Position chunkPosition)
                {
                    GenerationSettings sectorGenerationSettings = universe.SectorGenerationSettings;
                    size = sectorGenerationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + sector.sectorPosition.ChunkPosition.magnitude + chunkPosition.ChunkPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection();
                    RequestDensityMaps(universe, filament, sector, OnDensityMapsReceived);
                }
                #endregion

                #region Static Methods
                public static void ProcessDensityMapThreadInfoQueue()
                {
                    while (densityMapCoroutineInfoQueue.Count > 0)
                    {
                        DensityMapCoroutineInfo threadInfo = densityMapCoroutineInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);
                    }
                }
                #endregion
                
                #region Methods
                private void SampleDensityMaps(Universe universe, Filament filament, Sector sector, out Filament.Chunk.DensityMapCollection filamentDensityMaps, out DensityMapCollection sectorDensityMaps)
                {
                    GenerationSettings sectorGenerationSettings = universe.SectorGenerationSettings;
                    filamentDensityMaps = new Filament.Chunk.DensityMapCollection();
                    sectorDensityMaps = new DensityMapCollection();

                    for (int y = 0; y < sectorGenerationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < sectorGenerationSettings.ChunkSize; x++)
                        {
                            #region Filament Noise Sampling
                            float filamentOffsetX = -(1 / sectorGenerationSettings.ChunkAmount / sectorGenerationSettings.ChunkSize * x);
                            float filamentOffsetY = -(1 / sectorGenerationSettings.ChunkAmount / sectorGenerationSettings.ChunkSize * y);

                            float filamentSampleX = sector.SectorPosition.ChunkPosition.x + filamentOffsetX;
                            float filamentSampleY = sector.SectorPosition.ChunkPosition.y + filamentOffsetY;

                            float electronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float positronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float protonDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float antiProtonDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float neutronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);
                            float antiNeutronDensity = filament.SampleNoise(universe, filamentSampleX, filamentSampleY);

                            filamentDensityMaps.ElectronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), electronDensity);
                            filamentDensityMaps.PositronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), positronDensity);
                            filamentDensityMaps.ProtonDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), protonDensity);
                            filamentDensityMaps.AntiProtonDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), antiProtonDensity);
                            filamentDensityMaps.NeutronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), neutronDensity);
                            filamentDensityMaps.AntiNeutronDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), antiNeutronDensity);
                            #endregion

                            #region Sector Noise Sampling
                            float sectorOffsetX = -((sector.SectorPosition.ChunkPosition.x * sectorGenerationSettings.Size) + chunkPosition.ChunkPosition.x * sectorGenerationSettings.ChunkSize);
                            float sectorOffsetY = -((sector.SectorPosition.ChunkPosition.y * sectorGenerationSettings.Size) + chunkPosition.ChunkPosition.y * sectorGenerationSettings.ChunkSize);

                            float sectorSampleX = x + sectorOffsetX;
                            float sectorSampleY = y + sectorOffsetY;

                            float solidParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            float liquidParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            float gasParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            float plasmaParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);

                            sectorDensityMaps.SolidParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), solidParticleDensity);
                            sectorDensityMaps.LiquidParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), liquidParticleDensity);
                            sectorDensityMaps.GasParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), gasParticleDensity);
                            sectorDensityMaps.PlasmaParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), plasmaParticleDensity);
                            #endregion
                        }
                    }
                }

                private void RequestDensityMaps(Universe universe, Filament filament, Sector sector, Action<DensityMapCollection> callback)
                {
                    ParallelizationUtil.Instance.StartCoroutine(DensityMapGenerationCoroutine(universe, filament, sector, callback));
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
                    SampleDensityMaps(universe, filament, sector, out Filament.Chunk.DensityMapCollection filamentDensityMaps, out DensityMapCollection sectorDensityMaps);
                    ParallelizationUtil.RequestProcessedSectorDensityMaps(filamentDensityMaps, sectorDensityMaps, (sectorDensityMaps) =>
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
                universe.SectorDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                return universe.SectorNoiseGenerator.GetNoise(sampleX, sampleY);
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

                private struct DensityMapCoroutineInfo
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

                #region Classes
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

                    public DensityMapCollection()
                    {
                        MatterDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.Matter);
                        AntiMatterDensityMap = new DensityMap(new SerializableDictionary<Vector2Int, float>(), DensityMapType.AntiMatter);
                        GenerationState = GenerationState.Generating;
                    }
                }
                #endregion

                #region Static Fields
                private static Queue<DensityMapCoroutineInfo> densityMapCoroutineInfoQueue = new Queue<DensityMapCoroutineInfo>();
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
                public Chunk(Universe universe, Sector sector, Region region, Position chunkPosition)
                {
                    GenerationSettings regionGenerationSettings = universe.RegionGenerationSettings;
                    size = regionGenerationSettings.ChunkSize;
                    this.chunkPosition = chunkPosition;
                    chunkSeed = new SeededRandom((int)(universe.generationSettings.Seed + region.regionPosition.ChunkPosition.magnitude + chunkPosition.ChunkPosition.magnitude)).Range(int.MinValue, int.MaxValue);
                    densityMaps = new DensityMapCollection();
                    RequestDensityMaps(universe, sector, region, OnDensityMapsReceived);
                }
                #endregion

                #region Static Methods
                public static void ProcessDensityMapThreadInfoQueue()
                {
                    while (densityMapCoroutineInfoQueue.Count > 0)
                    {
                        DensityMapCoroutineInfo threadInfo = densityMapCoroutineInfoQueue.Dequeue();
                        threadInfo.Callback(threadInfo.DensityMaps);
                    }
                }
                #endregion

                #region Methods
                private void SampleDensityMaps(Universe universe, Sector sector, Region region, out Sector.Chunk.DensityMapCollection sectorDensityMaps, out DensityMapCollection regionDensityMaps)
                {
                    GenerationSettings regionGenerationSettings = universe.RegionGenerationSettings;
                    sectorDensityMaps = new Sector.Chunk.DensityMapCollection();
                    regionDensityMaps = new DensityMapCollection();

                    for (int y = 0; y < regionGenerationSettings.ChunkSize; y++)
                    {
                        for (int x = 0; x < regionGenerationSettings.ChunkSize; x++)
                        {
                            #region Sector Noise Sampling
                            float sectorOffsetX = -(1 / regionGenerationSettings.ChunkAmount / regionGenerationSettings.ChunkSize * x);
                            float sectorOffsetY = -(1 / regionGenerationSettings.ChunkAmount / regionGenerationSettings.ChunkSize * y);

                            float sectorSampleX = region.RegionPosition.ChunkPosition.x + sectorOffsetX;
                            float sectorSampleY = region.RegionPosition.ChunkPosition.y + sectorOffsetY;

                            float solidParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            float liquidParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            float gasParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);
                            float plasmaParticleDensity = sector.SampleNoise(universe, sectorSampleX, sectorSampleY);

                            sectorDensityMaps.SolidParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), solidParticleDensity);
                            sectorDensityMaps.LiquidParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), liquidParticleDensity);
                            sectorDensityMaps.GasParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), gasParticleDensity);
                            sectorDensityMaps.PlasmaParticleDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), plasmaParticleDensity);
                            #endregion

                            #region Region Noise Sampling
                            float regionOffsetX = -((region.RegionPosition.ChunkPosition.x * regionGenerationSettings.Size) + chunkPosition.ChunkPosition.x * regionGenerationSettings.ChunkSize);
                            float regionOffsetY = -((region.RegionPosition.ChunkPosition.y * regionGenerationSettings.Size) + chunkPosition.ChunkPosition.y * regionGenerationSettings.ChunkSize);

                            float regionSampleX = x + regionOffsetX;
                            float regionSampleY = y + regionOffsetY;

                            float matterDensity = region.SampleNoise(universe, regionSampleX, regionSampleY);
                            float antiMatterDensity = region.SampleNoise(universe, regionSampleX, regionSampleY);

                            regionDensityMaps.MatterDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), matterDensity);
                            regionDensityMaps.AntiMatterDensityMap.DensityMapDictionary.Add(new Vector2Int(x, y), antiMatterDensity);
                            #endregion
                        }
                    }
                }

                private void RequestDensityMaps(Universe universe, Sector sector, Region region, Action<DensityMapCollection> callback)
                {
                    ParallelizationUtil.Instance.StartCoroutine(DensityMapGenerationCoroutine(universe, sector, region, callback));
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
                    SampleDensityMaps(universe, sector, region, out Sector.Chunk.DensityMapCollection sectorDensityMaps, out DensityMapCollection regionDensityMaps);
                    ParallelizationUtil.RequestProcessedRegionDensityMaps(sectorDensityMaps, regionDensityMaps, (regionDensityMaps) =>
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
                universe.RegionDomainWarper.DomainWarp(ref sampleX, ref sampleY);
                return universe.RegionNoiseGenerator.GetNoise(sampleX, sampleY);
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
            Filament filament = GetFilament(sectorChunkPosition.FilamentPosition);
            Sector.Chunk sectorChunk = new Sector.Chunk(this, filament, sector, sectorChunkPosition);
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
            Sector sector = GetSector(regionChunkPosition.SectorPosition);
            Region.Chunk regionChunk = new Region.Chunk(this, sector, region, regionChunkPosition);
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