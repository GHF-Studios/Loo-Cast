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

    public class UniverseObserver : ExtendedMonoBehaviour
    {
        private int regionChunkLoadRadius = 2;
        private Universe currentUniverse;

        private Universe.Region.Position currentRegionPositionOffset;
        private List<Universe.Region.Chunk.Position> proximalRegionChunkPositions = new List<Universe.Region.Chunk.Position>();
        private List<Universe.Sector.Chunk.Position> proximalSectorChunkPositions = new List<Universe.Sector.Chunk.Position>();
        private List<Universe.Filament.Chunk.Position> proximalFilamentChunkPositions = new List<Universe.Filament.Chunk.Position>();
        private List<Universe.Region.Position> proximalRegionPositions = new List<Universe.Region.Position>();
        private List<Universe.Sector.Position> proximalSectorPositions = new List<Universe.Sector.Position>();
        private List<Universe.Filament.Position> proximalFilamentPositions = new List<Universe.Filament.Position>();

        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
            currentRegionPositionOffset = new Universe.Region.Position(Vector2Int.zero);
            
            Benchmark.Create("UpdateTotal");
            Benchmark.Create("UpdateRow");
            Benchmark.Create("UpdateElement");
            Benchmark.Create("UpdateRegion");
            Benchmark.Create("UpdateSector");
            Benchmark.Create("UpdateFilament");
            Benchmark.Create("UpdateRegionChunk");
            Benchmark.Create("UpdateSectorChunk");
            Benchmark.Create("UpdateFilamentChunk");
            Benchmark.Create("LoadTotal");
            Benchmark.Create("LoadRegion");
            Benchmark.Create("LoadSector");
            Benchmark.Create("LoadFilament");
            Benchmark.Create("LoadRegionChunk");
            Benchmark.Create("LoadSectorChunk");
            Benchmark.Create("LoadFilamentChunk");

            UpdateProximalPositions();
            LoadProximalPositions();
        }

        private void Update()
        {
            UpdateProximalPositions();
        }

        private void OnDrawGizmos()
        {
            DrawProximalPositionGizmos();
        }

        private void UpdateProximalPositions()
        {
            proximalRegionChunkPositions = new List<Universe.Region.Chunk.Position>();
            proximalSectorChunkPositions = new List<Universe.Sector.Chunk.Position>();
            proximalFilamentChunkPositions = new List<Universe.Filament.Chunk.Position>();
            proximalRegionPositions = new List<Universe.Region.Position>();
            proximalSectorPositions = new List<Universe.Sector.Position>();
            proximalFilamentPositions = new List<Universe.Filament.Position>();

            Universe.Region.Chunk.Position screenRegionChunkPosMin = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(0, 0)));
            screenRegionChunkPosMin = new Universe.Region.Chunk.Position(screenRegionChunkPosMin.VectorIntPosition - (Vector2Int.one * regionChunkLoadRadius));
            Universe.Region.Chunk.Position screenRegionChunkPosMax = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            screenRegionChunkPosMax = new Universe.Region.Chunk.Position(screenRegionChunkPosMax.VectorIntPosition + (Vector2Int.one * regionChunkLoadRadius));

            Benchmark.Start("UpdateTotal");
            for (int x = screenRegionChunkPosMin.VectorIntPosition.x; x <= screenRegionChunkPosMax.VectorIntPosition.x; x++)
            {
                Benchmark.Start("UpdateRow");
                for (int y = screenRegionChunkPosMin.VectorIntPosition.y; y <= screenRegionChunkPosMax.VectorIntPosition.y; y++)
                {
                    Benchmark.Start("UpdateElement");
                    Benchmark.Start("UpdateRegionChunk");
                    Universe.Region.Chunk.Position regionChunkPosition = new Universe.Region.Chunk.Position(new Vector2Int(x, y));
                    if (!proximalRegionChunkPositions.Contains(regionChunkPosition))
                    {
                        proximalRegionChunkPositions.Add(regionChunkPosition);
                    }
                    Benchmark.Stop("UpdateElement");
                    Benchmark.Stop("UpdateRegionChunk");

                    Benchmark.Start("UpdateElement");
                    Benchmark.Start("UpdateSectorChunk");
                    Universe.Sector.Chunk.Position sectorChunkPosition = new Universe.Sector.Chunk.Position(regionChunkPosition.WorldPosition);
                    if (!proximalSectorChunkPositions.Contains(sectorChunkPosition))
                    {
                        proximalSectorChunkPositions.Add(sectorChunkPosition);
                    }
                    Benchmark.Stop("UpdateElement");
                    Benchmark.Stop("UpdateSectorChunk");

                    Benchmark.Start("UpdateElement");
                    Benchmark.Start("UpdateFilamentChunk");
                    Universe.Filament.Chunk.Position filamentChunkPosition = new Universe.Filament.Chunk.Position(regionChunkPosition.WorldPosition);
                    if (!proximalFilamentChunkPositions.Contains(filamentChunkPosition))
                    {
                        proximalFilamentChunkPositions.Add(filamentChunkPosition);
                    }
                    Benchmark.Stop("UpdateElement");
                    Benchmark.Stop("UpdateFilamentChunk");

                    Benchmark.Start("UpdateElement");
                    Benchmark.Start("UpdateRegion");
                    Universe.Region.Position regionPosition = new Universe.Region.Position(regionChunkPosition.WorldPosition);
                    if (!proximalRegionPositions.Contains(regionPosition))
                    {
                        proximalRegionPositions.Add(regionPosition);
                    }
                    Benchmark.Stop("UpdateElement");
                    Benchmark.Stop("UpdateRegion");

                    Benchmark.Start("UpdateElement");
                    Benchmark.Start("UpdateSector");
                    Universe.Sector.Position sectorPosition = new Universe.Sector.Position(regionChunkPosition.WorldPosition);
                    if (!proximalSectorPositions.Contains(sectorPosition))
                    {
                        proximalSectorPositions.Add(sectorPosition);
                    }
                    Benchmark.Stop("UpdateElement");
                    Benchmark.Stop("UpdateSector");

                    Benchmark.Start("UpdateElement");
                    Benchmark.Start("UpdateFilament");
                    Universe.Filament.Position filamentPosition = new Universe.Filament.Position(regionChunkPosition.WorldPosition);
                    if (!proximalFilamentPositions.Contains(filamentPosition))
                    {
                        proximalFilamentPositions.Add(filamentPosition);
                    }
                    Benchmark.Stop("UpdateElement");
                    Benchmark.Stop("UpdateFilament");
                }
                Benchmark.Stop("UpdateRow");
            }
            Benchmark.Stop("UpdateTotal");
            
            UnityEngine.Debug.Log($"POSITION UPDATE:\t\t\t\tRegion: \t\t{Benchmark.AverageDuration("UpdateRegion").Microseconds()}({Benchmark.MaxDuration("UpdateRegion").Microseconds()})µs\t\t\tChunk: \t{Benchmark.AverageDuration("UpdateRegionChunk").Microseconds()}({Benchmark.MaxDuration("UpdateRegionChunk").Microseconds()})µs");
            UnityEngine.Debug.Log($"POSITION UPDATE:\t\t\t\tSector: \t\t{Benchmark.AverageDuration("UpdateSector").Microseconds()}({Benchmark.MaxDuration("UpdateSector").Microseconds()})µs\t\t\tChunk: \t{Benchmark.AverageDuration("UpdateSectorChunk").Microseconds()}({Benchmark.MaxDuration("UpdateSectorChunk").Microseconds()})µs");
            UnityEngine.Debug.Log($"POSITION UPDATE:\t\t\t\tFilament: \t{Benchmark.AverageDuration("UpdateFilament").Microseconds()}({Benchmark.MaxDuration("UpdateFilament").Microseconds()})µs\t\t\tChunk: \t{Benchmark.AverageDuration("UpdateFilamentChunk").Microseconds()}({Benchmark.MaxDuration("UpdateFilamentChunk").Microseconds()})µs");
            UnityEngine.Debug.Log($"POSITION UPDATE:\t\t\t\tElement: \t{Benchmark.AverageDuration("UpdateElement").Microseconds()}({Benchmark.MaxDuration("UpdateElement").Microseconds()})µs\t\t\tRow: \t{Benchmark.AverageDuration("UpdateRow").Microseconds()}({Benchmark.MaxDuration("UpdateRow").Microseconds()})µs\t\t\t Total: \t{Benchmark.AverageDuration("UpdateTotal").Microseconds()}({Benchmark.MaxDuration("UpdateTotal").Microseconds()})µs");
        }
        
        private void LoadProximalPositions()
        {
            Benchmark.Start("LoadTotal");

            Benchmark.Start("LoadFilament");
            foreach (Universe.Filament.Position proximalFilamentPosition in proximalFilamentPositions)
            {
                if (!currentUniverse.IsFilamentGenerated(proximalFilamentPosition))
                {
                    currentUniverse.GenerateFilament(proximalFilamentPosition);
                }
                else if (!currentUniverse.IsFilamentLoaded(proximalFilamentPosition))
                {
                    currentUniverse.LoadFilament(proximalFilamentPosition);
                }
            }
            Benchmark.Stop("LoadFilament");

            Benchmark.Start("LoadFilamentChunk");
            foreach (Universe.Filament.Chunk.Position proximalFilamentChunkPosition in proximalFilamentChunkPositions)
            {
                if (!currentUniverse.IsFilamentChunkGenerated(proximalFilamentChunkPosition))
                {
                    currentUniverse.GenerateFilamentChunk(proximalFilamentChunkPosition);
                }
                else if (!currentUniverse.IsFilamentChunkLoaded(proximalFilamentChunkPosition))
                {
                    currentUniverse.LoadFilamentChunk(proximalFilamentChunkPosition);
                }
            }
            Benchmark.Stop("LoadFilamentChunk");

            Benchmark.Start("LoadSector");
            foreach (Universe.Sector.Position proximalSectorPosition in proximalSectorPositions)
            {
                if (!currentUniverse.IsSectorGenerated(proximalSectorPosition))
                {
                    currentUniverse.GenerateSector(proximalSectorPosition);
                }
                else if (!currentUniverse.IsSectorLoaded(proximalSectorPosition))
                {
                    currentUniverse.LoadSector(proximalSectorPosition);
                }
            }
            Benchmark.Stop("LoadSector");

            Benchmark.Start("LoadSectorChunk");
            foreach (Universe.Sector.Chunk.Position proximalSectorChunkPosition in proximalSectorChunkPositions)
            {
                if (!currentUniverse.IsSectorChunkGenerated(proximalSectorChunkPosition))
                {
                    currentUniverse.GenerateSectorChunk(proximalSectorChunkPosition);
                }
                else if (!currentUniverse.IsSectorChunkLoaded(proximalSectorChunkPosition))
                {
                    currentUniverse.LoadSectorChunk(proximalSectorChunkPosition);
                }
            }
            Benchmark.Stop("LoadSectorChunk");

            Benchmark.Start("LoadRegion");
            foreach (Universe.Region.Position proximalRegionPosition in proximalRegionPositions)
            {
                if (!currentUniverse.IsRegionGenerated(proximalRegionPosition))
                {
                    currentUniverse.GenerateRegion(proximalRegionPosition);
                }
                else if (!currentUniverse.IsRegionLoaded(proximalRegionPosition))
                {
                    currentUniverse.LoadRegion(proximalRegionPosition);
                }
            }
            Benchmark.Stop("LoadRegion");

            Benchmark.Start("LoadRegionChunk");
            foreach (Universe.Region.Chunk.Position proximalRegionChunkPosition in proximalRegionChunkPositions)
            {
                if (!currentUniverse.IsRegionChunkGenerated(proximalRegionChunkPosition))
                {
                    currentUniverse.GenerateRegionChunk(proximalRegionChunkPosition);
                }
                else if (!currentUniverse.IsRegionChunkLoaded(proximalRegionChunkPosition))
                {
                    currentUniverse.LoadRegionChunk(proximalRegionChunkPosition);
                }
            }
            Benchmark.Stop("LoadRegionChunk");

            Benchmark.Stop("LoadTotal");

            UnityEngine.Debug.Log($"ELEMENT LOAD:\t\t\t\tRegion: \t\t{Benchmark.AverageDuration("LoadRegion").Milliseconds}({Benchmark.MaxDuration("LoadRegion").Milliseconds})ms\t\t\tChunk: \t{Benchmark.AverageDuration("LoadRegionChunk").Milliseconds}({Benchmark.MaxDuration("LoadRegionChunk").Milliseconds})ms");
            UnityEngine.Debug.Log($"ELEMENT LOAD:\t\t\t\tSector: \t\t{Benchmark.AverageDuration("LoadSector").Milliseconds}({Benchmark.MaxDuration("LoadSector").Milliseconds})ms\t\t\tChunk: \t{Benchmark.AverageDuration("LoadSectorChunk").Milliseconds}({Benchmark.MaxDuration("LoadSectorChunk").Milliseconds})ms");
            UnityEngine.Debug.Log($"ELEMENT LOAD:\t\t\t\tFilament: \t{Benchmark.AverageDuration("LoadFilament").Milliseconds}({Benchmark.MaxDuration("LoadFilament").Milliseconds})ms\t\t\tChunk: \t{Benchmark.AverageDuration("LoadFilamentChunk").Milliseconds}({Benchmark.MaxDuration("LoadFilamentChunk").Milliseconds})ms");
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
            foreach (Universe.Region.Chunk.Position proximalRegionChunkPosition in proximalRegionChunkPositions)
            {
                Gizmos.color = Color.green;
                Gizmos.DrawWireCube(proximalRegionChunkPosition.WorldPosition, new Vector2(regionChunkSize, regionChunkSize));
            }

            // Region
            foreach (Universe.Region.Position proximalRegionPosition in proximalRegionPositions)
            {
                Gizmos.color = Color.red;
                Gizmos.DrawWireCube(proximalRegionPosition.WorldPosition, new Vector2(regionSize, regionSize));
            }

            // Sector Chunk
            foreach (Universe.Sector.Chunk.Position proximalSectorChunkPosition in proximalSectorChunkPositions)
            {
                Gizmos.color = Color.yellow;
                Gizmos.DrawWireCube(proximalSectorChunkPosition.WorldPosition, new Vector2(sectorChunkSize, sectorChunkSize));
            }

            // Sector
            foreach (Universe.Sector.Position proximalSectorPosition in proximalSectorPositions)
            {
                Gizmos.color = Color.blue;
                Gizmos.DrawWireCube(proximalSectorPosition.WorldPosition, new Vector2(sectorSize, sectorSize));
            }

            // Filament Chunk
            foreach (Universe.Filament.Chunk.Position proximalFilamentChunkPosition in proximalFilamentChunkPositions)
            {
                Gizmos.color = new Color(1.0f, 0.0f, 1.0f);
                Gizmos.DrawWireCube(proximalFilamentChunkPosition.WorldPosition, new Vector2(filamentChunkSize, filamentChunkSize));
            }

            // Filament
            foreach (Universe.Filament.Position proximalFilamentPosition in proximalFilamentPositions)
            {
                Gizmos.color = new Color(1.0f, 0.5f, 0.0f);
                Gizmos.DrawWireCube(proximalFilamentPosition.WorldPosition, new Vector2(filamentSize, filamentSize));
            }
        }
    }
}
