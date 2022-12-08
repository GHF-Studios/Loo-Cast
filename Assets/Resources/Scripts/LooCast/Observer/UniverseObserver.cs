using System;
using System.Collections.Generic;
using System.Diagnostics;
using UnityEngine;

namespace LooCast.Observer
{
    using Core;
    using Game;
    using Universe;
    using Diagnostic;
    using Util;
    using System.Threading.Tasks;
    using Unity.VisualScripting;
    using System.Collections.Concurrent;

    public class UniverseObserver : ExtendedMonoBehaviour
    {
        private Universe currentUniverse;
        private int regionChunkLoadRadius;

        private ConcurrentDictionary<Universe.Region.Chunk.Position, byte> proximalRegionChunkPositions = new ConcurrentDictionary<Universe.Region.Chunk.Position, byte>();
        private ConcurrentDictionary<Universe.Sector.Chunk.Position, byte> proximalSectorChunkPositions = new ConcurrentDictionary<Universe.Sector.Chunk.Position, byte>();
        private ConcurrentDictionary<Universe.Filament.Chunk.Position, byte> proximalFilamentChunkPositions = new ConcurrentDictionary<Universe.Filament.Chunk.Position, byte>();
        private ConcurrentDictionary<Universe.Region.Position, byte> proximalRegionPositions = new ConcurrentDictionary<Universe.Region.Position, byte>();
        private ConcurrentDictionary<Universe.Sector.Position, byte> proximalSectorPositions = new ConcurrentDictionary<Universe.Sector.Position, byte>();
        private ConcurrentDictionary<Universe.Filament.Position, byte> proximalFilamentPositions = new ConcurrentDictionary<Universe.Filament.Position, byte>();

        private ConcurrentDictionary<Universe.Region.Chunk.Position, byte> previouslyProximalRegionChunkPositions = new ConcurrentDictionary<Universe.Region.Chunk.Position, byte>();
        private ConcurrentDictionary<Universe.Sector.Chunk.Position, byte> previouslyProximalSectorChunkPositions = new ConcurrentDictionary<Universe.Sector.Chunk.Position, byte>();
        private ConcurrentDictionary<Universe.Filament.Chunk.Position, byte> previouslyProximalFilamentChunkPositions = new ConcurrentDictionary<Universe.Filament.Chunk.Position, byte>();
        private ConcurrentDictionary<Universe.Region.Position, byte> previouslyProximalRegionPositions = new ConcurrentDictionary<Universe.Region.Position, byte>();
        private ConcurrentDictionary<Universe.Sector.Position, byte> previouslyProximalSectorPositions = new ConcurrentDictionary<Universe.Sector.Position, byte>();
        private ConcurrentDictionary<Universe.Filament.Position, byte> previouslyProximalFilamentPositions = new ConcurrentDictionary<Universe.Filament.Position, byte>();

        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
            regionChunkLoadRadius = 2;

            Benchmark.Create("UpdatePositions");
            Benchmark.Create("UpdatePosition");
            Benchmark.Create("UpdateRegion");
            Benchmark.Create("UpdateSector");
            Benchmark.Create("UpdateFilament");
            Benchmark.Create("UpdateRegionChunk");
            Benchmark.Create("UpdateSectorChunk");
            Benchmark.Create("UpdateFilamentChunk");
            Benchmark.Create("LoadPositions");
            Benchmark.Create("LoadRegion");
            Benchmark.Create("LoadSector");
            Benchmark.Create("LoadFilament");
            Benchmark.Create("LoadRegionChunk");
            Benchmark.Create("LoadSectorChunk");
            Benchmark.Create("LoadFilamentChunk");
            Benchmark.Create("UnloadPositions");

            UpdateProximalPositions();
            LoadProximalPositions();
            UnloadPreviouslyProximalPositions();
        }

        private void Update()
        {
            UpdateProximalPositions();
            UpdatePreviouslyProximalPositions();
            LoadProximalPositions();
            UnloadPreviouslyProximalPositions();
            PrintBenchmarks();
        }

        private void OnDrawGizmos()
        {
            DrawProximalPositionGizmos();
        }

        private void UpdateProximalPositions()
        {
            if (proximalRegionChunkPositions == null)
            {
                proximalRegionChunkPositions = new ConcurrentDictionary<Universe.Region.Chunk.Position, byte>();
            }
            if (proximalSectorChunkPositions == null)
            {
                proximalSectorChunkPositions = new ConcurrentDictionary<Universe.Sector.Chunk.Position, byte>();
            }
            if (proximalFilamentChunkPositions == null)
            {
                proximalFilamentChunkPositions = new ConcurrentDictionary<Universe.Filament.Chunk.Position, byte>();
            }
            if (proximalRegionPositions == null)
            {
                proximalRegionPositions = new ConcurrentDictionary<Universe.Region.Position, byte>();
            }
            if (proximalSectorPositions == null)
            {
                proximalSectorPositions = new ConcurrentDictionary<Universe.Sector.Position, byte>();
            }
            if (proximalFilamentPositions == null)
            {
                proximalFilamentPositions = new ConcurrentDictionary<Universe.Filament.Position, byte>();
            }

            Universe.Region.Chunk.Position screenRegionChunkPosMin = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(0, 0)));
            screenRegionChunkPosMin = new Universe.Region.Chunk.Position(screenRegionChunkPosMin.VectorIntPosition - (Vector2Int.one * regionChunkLoadRadius));
            Universe.Region.Chunk.Position screenRegionChunkPosMax = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            screenRegionChunkPosMax = new Universe.Region.Chunk.Position(screenRegionChunkPosMax.VectorIntPosition + (Vector2Int.one * regionChunkLoadRadius));

            Benchmark.Start("UpdatePositions");
            int screenRegionChunkPosWidth = screenRegionChunkPosMax.VectorIntPosition.x - screenRegionChunkPosMin.VectorIntPosition.x;
            Parallel.For(screenRegionChunkPosMin.VectorIntPosition.x, screenRegionChunkPosMax.VectorIntPosition.x + 1, (x) =>
            {
                Parallel.For(screenRegionChunkPosMin.VectorIntPosition.y, screenRegionChunkPosMax.VectorIntPosition.y + 1, (y) =>
                {
                    int threadID = screenRegionChunkPosWidth * y + x;
                    Benchmark.Start("UpdatePosition", threadID);
                    Benchmark.Start("UpdateRegionChunk", threadID);
                    Universe.Region.Chunk.Position regionChunkPosition = new Universe.Region.Chunk.Position(new Vector2Int(x, y));
                    if (!proximalRegionChunkPositions.ContainsKey(regionChunkPosition) && !currentUniverse.IsRegionChunkLoaded(regionChunkPosition))
                    {
                        proximalRegionChunkPositions.TryAdd(regionChunkPosition, default(byte));
                    }
                    Benchmark.Stop("UpdatePosition", threadID);
                    Benchmark.Stop("UpdateRegionChunk", threadID);

                    Benchmark.Start("UpdatePosition", threadID);
                    Benchmark.Start("UpdateSectorChunk", threadID);
                    Universe.Sector.Chunk.Position sectorChunkPosition = new Universe.Sector.Chunk.Position(regionChunkPosition.WorldPosition);
                    if (!proximalSectorChunkPositions.ContainsKey(sectorChunkPosition) && !currentUniverse.IsSectorChunkLoaded(sectorChunkPosition))
                    {
                        proximalSectorChunkPositions.TryAdd(sectorChunkPosition, default(byte));
                    }
                    Benchmark.Stop("UpdatePosition", threadID);
                    Benchmark.Stop("UpdateSectorChunk", threadID);

                    Benchmark.Start("UpdatePosition", threadID);
                    Benchmark.Start("UpdateFilamentChunk", threadID);
                    Universe.Filament.Chunk.Position filamentChunkPosition = new Universe.Filament.Chunk.Position(regionChunkPosition.WorldPosition);
                    if (!proximalFilamentChunkPositions.ContainsKey(filamentChunkPosition) && !currentUniverse.IsFilamentChunkLoaded(filamentChunkPosition))
                    {
                        proximalFilamentChunkPositions.TryAdd(filamentChunkPosition, default(byte));
                    }
                    Benchmark.Stop("UpdatePosition", threadID);
                    Benchmark.Stop("UpdateFilamentChunk", threadID);

                    Benchmark.Start("UpdatePosition", threadID);
                    Benchmark.Start("UpdateRegion", threadID);
                    Universe.Region.Position regionPosition = new Universe.Region.Position(regionChunkPosition.WorldPosition);
                    if (!proximalRegionPositions.ContainsKey(regionPosition) && !currentUniverse.IsRegionLoaded(regionPosition))
                    {
                        proximalRegionPositions.TryAdd(regionPosition, default(byte));
                    }
                    Benchmark.Stop("UpdatePosition", threadID);
                    Benchmark.Stop("UpdateRegion", threadID);

                    Benchmark.Start("UpdatePosition", threadID);
                    Benchmark.Start("UpdateSector", threadID);
                    Universe.Sector.Position sectorPosition = new Universe.Sector.Position(regionChunkPosition.WorldPosition);
                    if (!proximalSectorPositions.ContainsKey(sectorPosition) && !currentUniverse.IsSectorLoaded(sectorPosition))
                    {
                        proximalSectorPositions.TryAdd(sectorPosition, default(byte));
                    }
                    Benchmark.Stop("UpdatePosition", threadID);
                    Benchmark.Stop("UpdateSector", threadID);

                    Benchmark.Start("UpdatePosition", threadID);
                    Benchmark.Start("UpdateFilament", threadID);
                    Universe.Filament.Position filamentPosition = new Universe.Filament.Position(regionChunkPosition.WorldPosition);
                    if (!proximalFilamentPositions.ContainsKey(filamentPosition) && !currentUniverse.IsFilamentLoaded(filamentPosition))
                    {
                        proximalFilamentPositions.TryAdd(filamentPosition, default(byte));
                    }
                    Benchmark.Stop("UpdatePosition", threadID);
                    Benchmark.Stop("UpdateFilament", threadID);
                });
            });
            Benchmark.Stop("UpdatePositions");
        }

        private void UpdatePreviouslyProximalPositions()
        {
            previouslyProximalRegionChunkPositions = new ConcurrentDictionary<Universe.Region.Chunk.Position, byte>();
            previouslyProximalSectorChunkPositions = new ConcurrentDictionary<Universe.Sector.Chunk.Position, byte>();
            previouslyProximalFilamentChunkPositions = new ConcurrentDictionary<Universe.Filament.Chunk.Position, byte>();
            previouslyProximalRegionPositions = new ConcurrentDictionary<Universe.Region.Position, byte>();
            previouslyProximalSectorPositions = new ConcurrentDictionary<Universe.Sector.Position, byte>();
            previouslyProximalFilamentPositions = new ConcurrentDictionary<Universe.Filament.Position, byte>();

            foreach (Universe.Region.Chunk.Position loadedRegionChunkPosition in currentUniverse.LoadedRegionChunkPositions)
            {
                if (!proximalRegionChunkPositions.ContainsKey(loadedRegionChunkPosition))
                {
                    previouslyProximalRegionChunkPositions.TryAdd(loadedRegionChunkPosition, default(byte));
                }
            }

            foreach (Universe.Region.Position loadedRegionPosition in currentUniverse.LoadedRegionPositions)
            {
                if (!proximalRegionPositions.ContainsKey(loadedRegionPosition))
                {
                    previouslyProximalRegionPositions.TryAdd(loadedRegionPosition, default(byte));
                }
            }

            foreach (Universe.Sector.Chunk.Position loadedSectorChunkPosition in currentUniverse.LoadedSectorChunkPositions)
            {
                if (!proximalSectorChunkPositions.ContainsKey(loadedSectorChunkPosition))
                {
                    previouslyProximalSectorChunkPositions.TryAdd(loadedSectorChunkPosition, default(byte));
                }
            }

            foreach (Universe.Sector.Position loadedSectorPosition in currentUniverse.LoadedSectorPositions)
            {
                if (!proximalSectorPositions.ContainsKey(loadedSectorPosition))
                {
                    previouslyProximalSectorPositions.TryAdd(loadedSectorPosition, default(byte));
                }
            }

            foreach (Universe.Filament.Chunk.Position loadedFilamentChunkPosition in currentUniverse.LoadedFilamentChunkPositions)
            {
                if (!proximalFilamentChunkPositions.ContainsKey(loadedFilamentChunkPosition))
                {
                    previouslyProximalFilamentChunkPositions.TryAdd(loadedFilamentChunkPosition, default(byte));
                }
            }

            foreach (Universe.Filament.Position loadedFilamentPosition in currentUniverse.LoadedFilamentPositions)
            {
                if (!proximalFilamentPositions.ContainsKey(loadedFilamentPosition))
                {
                    previouslyProximalFilamentPositions.TryAdd(loadedFilamentPosition, default(byte));
                }
            }
        }
        
        private void LoadProximalPositions()
        {
            Benchmark.Start("LoadPositions");

            Benchmark.Start("LoadFilament");
            foreach (var proximalFilamentPositionKeyValuePair in proximalFilamentPositions)
            {
                if (!currentUniverse.IsFilamentGenerated(proximalFilamentPositionKeyValuePair.Key))
                {
                    currentUniverse.GenerateFilament(proximalFilamentPositionKeyValuePair.Key);
                }
                else if (!currentUniverse.IsFilamentLoaded(proximalFilamentPositionKeyValuePair.Key))
                {
                    currentUniverse.LoadFilament(proximalFilamentPositionKeyValuePair.Key);
                }
            }
            Benchmark.Stop("LoadFilament");

            Benchmark.Start("LoadFilamentChunk");
            foreach (var proximalFilamentChunkPositionKeyValuePair in proximalFilamentChunkPositions)
            {
                if (!currentUniverse.IsFilamentChunkGenerated(proximalFilamentChunkPositionKeyValuePair.Key))
                {
                    currentUniverse.GenerateFilamentChunk(proximalFilamentChunkPositionKeyValuePair.Key);
                }
                else if (!currentUniverse.IsFilamentChunkLoaded(proximalFilamentChunkPositionKeyValuePair.Key))
                {
                    currentUniverse.LoadFilamentChunk(proximalFilamentChunkPositionKeyValuePair.Key);
                }
            }
            Benchmark.Stop("LoadFilamentChunk");

            Benchmark.Start("LoadSector");
            foreach (var proximalSectorPositionKeyValuePair in proximalSectorPositions)
            {
                if (!currentUniverse.IsSectorGenerated(proximalSectorPositionKeyValuePair.Key))
                {
                    currentUniverse.GenerateSector(proximalSectorPositionKeyValuePair.Key);
                }
                else if (!currentUniverse.IsSectorLoaded(proximalSectorPositionKeyValuePair.Key))
                {
                    currentUniverse.LoadSector(proximalSectorPositionKeyValuePair.Key);
                }
            }
            Benchmark.Stop("LoadSector");

            Benchmark.Start("LoadSectorChunk");
            foreach (var proximalSectorChunkPositionKeyValuePair in proximalSectorChunkPositions)
            {
                if (!currentUniverse.IsSectorChunkGenerated(proximalSectorChunkPositionKeyValuePair.Key))
                {
                    currentUniverse.GenerateSectorChunk(proximalSectorChunkPositionKeyValuePair.Key);
                }
                else if (!currentUniverse.IsSectorChunkLoaded(proximalSectorChunkPositionKeyValuePair.Key))
                {
                    currentUniverse.LoadSectorChunk(proximalSectorChunkPositionKeyValuePair.Key);
                }
            }
            Benchmark.Stop("LoadSectorChunk");

            Benchmark.Start("LoadRegion");
            foreach (var proximalRegionPositionKeyValuePair in proximalRegionPositions)
            {
                if (!currentUniverse.IsRegionGenerated(proximalRegionPositionKeyValuePair.Key))
                {
                    currentUniverse.GenerateRegion(proximalRegionPositionKeyValuePair.Key);
                }
                else if (!currentUniverse.IsRegionLoaded(proximalRegionPositionKeyValuePair.Key))
                {
                    currentUniverse.LoadRegion(proximalRegionPositionKeyValuePair.Key);
                }
            }
            Benchmark.Stop("LoadRegion");

            Benchmark.Start("LoadRegionChunk");
            foreach (var proximalRegionChunkPositionKeyValuePair in proximalRegionChunkPositions)
            {
                if (!currentUniverse.IsRegionChunkGenerated(proximalRegionChunkPositionKeyValuePair.Key))
                {
                    currentUniverse.GenerateRegionChunk(proximalRegionChunkPositionKeyValuePair.Key);
                }
                else if (!currentUniverse.IsRegionChunkLoaded(proximalRegionChunkPositionKeyValuePair.Key))
                {
                    currentUniverse.LoadRegionChunk(proximalRegionChunkPositionKeyValuePair.Key);
                }
            }
            Benchmark.Stop("LoadRegionChunk");

            Benchmark.Stop("LoadPositions");
        }

        private void UnloadPreviouslyProximalPositions()
        {
            Benchmark.Start("UnloadPositions");

            // Region Chunk
            foreach (Universe.Region.Chunk.Position previouslyProximalRegionChunkPosition in previouslyProximalRegionChunkPositions.Keys)
            {
                if (currentUniverse.IsRegionChunkLoaded(previouslyProximalRegionChunkPosition))
                {
                    currentUniverse.UnloadRegionChunk(previouslyProximalRegionChunkPosition);
                }
            }

            // Region
            foreach (Universe.Region.Position previouslyProximalRegionPosition in previouslyProximalRegionPositions.Keys)
            {
                if (currentUniverse.IsRegionLoaded(previouslyProximalRegionPosition))
                {
                    currentUniverse.UnloadRegion(previouslyProximalRegionPosition);
                }
            }

            // Sector Chunk
            foreach (Universe.Sector.Chunk.Position previouslyProximalSectorChunkPosition in previouslyProximalSectorChunkPositions.Keys)
            {
                if (currentUniverse.IsSectorChunkLoaded(previouslyProximalSectorChunkPosition))
                {
                    currentUniverse.UnloadSectorChunk(previouslyProximalSectorChunkPosition);
                }
            }

            // Sector
            foreach (Universe.Sector.Position previouslyProximalSectorPosition in previouslyProximalSectorPositions.Keys)
            {
                if (currentUniverse.IsSectorLoaded(previouslyProximalSectorPosition))
                {
                    currentUniverse.UnloadSector(previouslyProximalSectorPosition);
                }
            }

            // Filament Chunk
            foreach (Universe.Filament.Chunk.Position previouslyProximalFilamentChunkPosition in previouslyProximalFilamentChunkPositions.Keys)
            {
                if (currentUniverse.IsFilamentChunkLoaded(previouslyProximalFilamentChunkPosition))
                {
                    currentUniverse.UnloadFilamentChunk(previouslyProximalFilamentChunkPosition);
                }
            }

            // Filament
            foreach (Universe.Filament.Position previouslyProximalFilamentPosition in previouslyProximalFilamentPositions.Keys)
            {
                if (currentUniverse.IsFilamentLoaded(previouslyProximalFilamentPosition))
                {
                    currentUniverse.UnloadFilament(previouslyProximalFilamentPosition);
                }
            }

            Benchmark.Stop("UnloadPositions");
        }

        private void DrawProximalPositionGizmos()
        {
            int regionSize = currentUniverse.RegionGenerationSettings.Size;
            int regionChunkSize = currentUniverse.RegionGenerationSettings.ChunkSize;
            int sectorSize = currentUniverse.SectorGenerationSettings.Size;
            int sectorChunkSize = currentUniverse.SectorGenerationSettings.ChunkSize;
            int filamentSize = currentUniverse.FilamentGenerationSettings.Size;
            int filamentChunkSize = currentUniverse.FilamentGenerationSettings.ChunkSize;

            // Region Chunk
            foreach (Universe.Region.Chunk.Position loadedRegionChunkPositions in currentUniverse.LoadedRegionChunkPositions)
            {
                Gizmos.color = Color.green;
                Gizmos.DrawWireCube(loadedRegionChunkPositions.WorldPosition, new Vector2(regionChunkSize, regionChunkSize));
            }

            // Region
            foreach (Universe.Region.Position loadedRegionPositions in currentUniverse.LoadedRegionPositions)
            {
                Gizmos.color = Color.red;
                Gizmos.DrawWireCube(loadedRegionPositions.WorldPosition, new Vector2(regionSize, regionSize));
            }

            // Sector Chunk
            foreach (Universe.Sector.Chunk.Position loadedSectorChunkPositions in currentUniverse.LoadedSectorChunkPositions)
            {
                Gizmos.color = Color.yellow;
                Gizmos.DrawWireCube(loadedSectorChunkPositions.WorldPosition, new Vector2(sectorChunkSize, sectorChunkSize));
            }

            // Sector
            foreach (Universe.Sector.Position loadedSectorPositions in currentUniverse.LoadedSectorPositions)
            {
                Gizmos.color = Color.blue;
                Gizmos.DrawWireCube(loadedSectorPositions.WorldPosition, new Vector2(sectorSize, sectorSize));
            }

            // Filament Chunk
            foreach (Universe.Filament.Chunk.Position loadedFilamentChunkPositions in currentUniverse.LoadedFilamentChunkPositions)
            {
                Gizmos.color = new Color(1.0f, 0.0f, 1.0f);
                Gizmos.DrawWireCube(loadedFilamentChunkPositions.WorldPosition, new Vector2(filamentChunkSize, filamentChunkSize));
            }

            // Filament
            foreach (Universe.Filament.Position loadedFilamentPositions in currentUniverse.LoadedFilamentPositions)
            {
                Gizmos.color = new Color(1.0f, 0.5f, 0.0f);
                Gizmos.DrawWireCube(loadedFilamentPositions.WorldPosition, new Vector2(filamentSize, filamentSize));
            }
        }

        private void PrintBenchmarks()
        {
            UnityEngine.Debug.Log(
                $"POSITION UPDATE:" +
                $"\t\t\t\tRegion: \t\t{Benchmark.AverageDuration("UpdateRegion").Microseconds()}({Benchmark.MaxDuration("UpdateRegion").Microseconds()})µs" +
                $"\t\t\tChunk: \t{Benchmark.AverageDuration("UpdateRegionChunk").Microseconds()}({Benchmark.MaxDuration("UpdateRegionChunk").Microseconds()})µs");
            UnityEngine.Debug.Log(
                $"POSITION UPDATE:" +
                $"\t\t\t\tSector: \t\t{Benchmark.AverageDuration("UpdateSector").Microseconds()}({Benchmark.MaxDuration("UpdateSector").Microseconds()})µs" +
                $"\t\t\tChunk: \t{Benchmark.AverageDuration("UpdateSectorChunk").Microseconds()}({Benchmark.MaxDuration("UpdateSectorChunk").Microseconds()})µs");
            UnityEngine.Debug.Log(
                $"POSITION UPDATE:" +
                $"\t\t\t\tFilament: \t{Benchmark.AverageDuration("UpdateFilament").Microseconds()}({Benchmark.MaxDuration("UpdateFilament").Microseconds()})µs" +
                $"\t\t\tChunk: \t{Benchmark.AverageDuration("UpdateFilamentChunk").Microseconds()}({Benchmark.MaxDuration("UpdateFilamentChunk").Microseconds()})µs");
            UnityEngine.Debug.Log(
                $"ELEMENT LOAD:" +
                $"\t\t\t\tRegion: \t\t{Benchmark.AverageDuration("LoadRegion").Milliseconds}({Benchmark.MaxDuration("LoadRegion").Milliseconds})ms" +
                $"\t\t\t\tChunk: \t{Benchmark.AverageDuration("LoadRegionChunk").Milliseconds}({Benchmark.MaxDuration("LoadRegionChunk").Milliseconds})ms");
            UnityEngine.Debug.Log(
                $"ELEMENT LOAD:" +
                $"\t\t\t\tSector: \t\t{Benchmark.AverageDuration("LoadSector").Milliseconds}({Benchmark.MaxDuration("LoadSector").Milliseconds})ms" +
                $"\t\t\t\tChunk: \t{Benchmark.AverageDuration("LoadSectorChunk").Milliseconds}({Benchmark.MaxDuration("LoadSectorChunk").Milliseconds})ms");
            UnityEngine.Debug.Log(
                $"ELEMENT LOAD:" +
                $"\t\t\t\tFilament: \t{Benchmark.AverageDuration("LoadFilament").Milliseconds}({Benchmark.MaxDuration("LoadFilament").Milliseconds})ms" +
                $"\t\t\t\tChunk: \t{Benchmark.AverageDuration("LoadFilamentChunk").Milliseconds}({Benchmark.MaxDuration("LoadFilamentChunk").Milliseconds})ms");
            UnityEngine.Debug.Log(
                $"MISCELLANEOUS:" +
                $"\t\t\t\tUpdate Position: \t{Benchmark.AverageDuration("UpdatePosition").Milliseconds}({Benchmark.MaxDuration("UpdatePosition").Milliseconds})ms" +
                $"\t\t Update Positions: \t{Benchmark.AverageDuration("UpdatePositions").Milliseconds}({Benchmark.MaxDuration("UpdatePositions").Milliseconds})ms" +
                $"\t\t Load Positions: \t{Benchmark.AverageDuration("LoadPositions").Milliseconds}({Benchmark.MaxDuration("LoadPositions").Milliseconds})ms" +
                $"\t\t Unload Positions: \t{Benchmark.AverageDuration("UnloadPositions").Milliseconds}({Benchmark.MaxDuration("UnloadPositions").Milliseconds})ms");
        }
    }
}
