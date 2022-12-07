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
            Stopwatch stopwatch = new Stopwatch();
            
            Benchmark.Create("Total");
            Benchmark.Create("Row");
            Benchmark.Create("Element");
            Benchmark.Create("Region");
            Benchmark.Create("Sector");
            Benchmark.Create("Filament");
            Benchmark.Create("RegionChunk");
            Benchmark.Create("SectorChunk");
            Benchmark.Create("FilamentChunk");

            UpdateProximalPositions();

            
            stopwatch.Start();
            #region Filament Loading
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
            #endregion
            stopwatch.Stop();
            UnityEngine.Debug.Log($"[UniverseObserver] Took {stopwatch.ElapsedMilliseconds}ms to Load Filaments!");

            stopwatch.Restart();
            #region Sector Loading
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
            #endregion
            stopwatch.Stop();
            UnityEngine.Debug.Log($"[UniverseObserver] Took {stopwatch.ElapsedMilliseconds}ms to Load Sectors!");

            stopwatch.Restart();
            #region Region Loading
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
            #endregion
            stopwatch.Stop();
            UnityEngine.Debug.Log($"[UniverseObserver] Took {stopwatch.ElapsedMilliseconds}ms to Load Regions!");
            
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
            GetProximalPositions(out proximalRegionChunkPositions, out proximalSectorChunkPositions, out proximalFilamentChunkPositions, out proximalRegionPositions, out proximalSectorPositions, out proximalFilamentPositions);
        }
        
        private void TranslateRegions(Vector2Int currentRegionPosition)
        {
            Vector2Int currentRegionPositionDifference = currentRegionPosition - currentRegionPositionOffset.VectorIntPosition;
            int threshold = 1;
            if (Mathf.Abs(currentRegionPositionDifference.x) > threshold || Mathf.Abs(currentRegionPositionDifference.y) > threshold)
            {
                int regionSize = currentUniverse.RegionGenerationSettings.Size;
                currentRegionPositionOffset = new Universe.Region.Position(currentRegionPositionOffset.VectorIntPosition + currentRegionPositionDifference);
                transform.Translate(-new Vector3(currentRegionPositionDifference.x * regionSize, currentRegionPositionDifference.y * regionSize));
            }
        }

        private void GetProximalPositions(out List<Universe.Region.Chunk.Position> regionChunkPositions, out List<Universe.Sector.Chunk.Position> sectorChunkPositions, out List<Universe.Filament.Chunk.Position> filamentChunkPositions, out List<Universe.Region.Position> regionPositions, out List<Universe.Sector.Position> sectorPositions, out List<Universe.Filament.Position> filamentPositions)
        {
            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(transform.position);

            regionChunkPositions = new List<Universe.Region.Chunk.Position>();
            sectorChunkPositions = new List<Universe.Sector.Chunk.Position>();
            filamentChunkPositions = new List<Universe.Filament.Chunk.Position>();
            regionPositions = new List<Universe.Region.Position>();
            sectorPositions = new List<Universe.Sector.Position>();
            filamentPositions = new List<Universe.Filament.Position>();

            #region DEV
            /*
            regionChunkPositions.Add(currentRegionChunkPosition);
            sectorChunkPositions.Add(new Universe.Sector.Chunk.Position(currentUniverse, currentRegionChunkPosition.WorldPosition));
            filamentChunkPositions.Add(new Universe.Filament.Chunk.Position(currentUniverse, currentRegionChunkPosition.WorldPosition));
            regionPositions.Add(currentRegionChunkPosition.RegionPosition);
            sectorPositions.Add(new Universe.Sector.Position(currentUniverse, currentRegionChunkPosition.WorldPosition));
            filamentPositions.Add(new Universe.Filament.Position(currentUniverse, currentRegionChunkPosition.WorldPosition));

            return;
            */
            #endregion

            Universe.Region.Chunk.Position screenRegionChunkPosMin = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(0, 0)));
            screenRegionChunkPosMin = new Universe.Region.Chunk.Position(screenRegionChunkPosMin.VectorIntPosition - (Vector2Int.one * regionChunkLoadRadius));
            Universe.Region.Chunk.Position screenRegionChunkPosMax = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            screenRegionChunkPosMax = new Universe.Region.Chunk.Position(screenRegionChunkPosMax.VectorIntPosition + (Vector2Int.one * regionChunkLoadRadius));

            Benchmark.Start("Total");
            for (int x = screenRegionChunkPosMin.VectorIntPosition.x; x <= screenRegionChunkPosMax.VectorIntPosition.x; x++)
            {
                Benchmark.Start("Row");
                for (int y = screenRegionChunkPosMin.VectorIntPosition.y; y <= screenRegionChunkPosMax.VectorIntPosition.y; y++)
                {
                    Benchmark.Start("Element");
                    Benchmark.Start("RegionChunk");
                    Universe.Region.Chunk.Position regionChunkPosition = new Universe.Region.Chunk.Position(new Vector2Int(x, y));
                    if (!regionChunkPositions.Contains(regionChunkPosition))
                    {
                        regionChunkPositions.Add(regionChunkPosition);
                    }
                    Benchmark.Stop("Element");
                    Benchmark.Stop("RegionChunk");

                    Benchmark.Start("Element");
                    Benchmark.Start("SectorChunk");
                    Universe.Sector.Chunk.Position sectorChunkPosition = new Universe.Sector.Chunk.Position(regionChunkPosition.WorldPosition);
                    if (!sectorChunkPositions.Contains(sectorChunkPosition))
                    {
                        sectorChunkPositions.Add(sectorChunkPosition);
                    }
                    Benchmark.Stop("Element");
                    Benchmark.Stop("SectorChunk");

                    Benchmark.Start("Element");
                    Benchmark.Start("FilamentChunk");
                    Universe.Filament.Chunk.Position filamentChunkPosition = new Universe.Filament.Chunk.Position(regionChunkPosition.WorldPosition);
                    if (!filamentChunkPositions.Contains(filamentChunkPosition))
                    {
                        filamentChunkPositions.Add(filamentChunkPosition);
                    }
                    Benchmark.Stop("Element");
                    Benchmark.Stop("FilamentChunk");

                    Benchmark.Start("Element");
                    Benchmark.Start("Region");
                    Universe.Region.Position regionPosition = new Universe.Region.Position(regionChunkPosition.WorldPosition);
                    if (!regionPositions.Contains(regionPosition))
                    {
                        regionPositions.Add(regionPosition);
                    }
                    Benchmark.Stop("Element");
                    Benchmark.Stop("Region");

                    Benchmark.Start("Element");
                    Benchmark.Start("Sector");
                    Universe.Sector.Position sectorPosition = new Universe.Sector.Position(regionChunkPosition.WorldPosition);
                    if (!sectorPositions.Contains(sectorPosition))
                    {
                        sectorPositions.Add(sectorPosition);
                    }
                    Benchmark.Stop("Element");
                    Benchmark.Stop("Sector");

                    Benchmark.Start("Element");
                    Benchmark.Start("Filament");
                    Universe.Filament.Position filamentPosition = new Universe.Filament.Position(regionChunkPosition.WorldPosition);
                    if (!filamentPositions.Contains(filamentPosition))
                    {
                        filamentPositions.Add(filamentPosition);
                    }
                    Benchmark.Stop("Element");
                    Benchmark.Stop("Filament");
                }
                Benchmark.Stop("Row");
            }
            Benchmark.Stop("Total");
            
            UnityEngine.Debug.Log($"\t\t\t\tRegion: \t\t{Benchmark.AverageDuration("Region").Microseconds()}({Benchmark.MaxDuration("Region").Microseconds()})µs\t\t\tChunk: \t{Benchmark.AverageDuration("RegionChunk").Microseconds()}({Benchmark.MaxDuration("RegionChunk").Microseconds()})µs");
            UnityEngine.Debug.Log($"\t\t\t\tSector: \t\t{Benchmark.AverageDuration("Sector").Microseconds()}({Benchmark.MaxDuration("Sector").Microseconds()})µs\t\t\tChunk: \t{Benchmark.AverageDuration("SectorChunk").Microseconds()}({Benchmark.MaxDuration("SectorChunk").Microseconds()})µs");
            UnityEngine.Debug.Log($"\t\t\t\tFilament: \t{Benchmark.AverageDuration("Filament").Microseconds()}({Benchmark.MaxDuration("Filament").Microseconds()})µs\t\t\tChunk: \t{Benchmark.AverageDuration("FilamentChunk").Microseconds()}({Benchmark.MaxDuration("FilamentChunk").Microseconds()})µs");
            UnityEngine.Debug.Log($"\t\t\t\tElement: \t{Benchmark.AverageDuration("Element").Microseconds()}({Benchmark.MaxDuration("Element").Microseconds()})µs\t\t\tRow: \t{Benchmark.AverageDuration("Row").Microseconds()}({Benchmark.MaxDuration("Row").Microseconds()})µs\t\t\t Total: \t{Benchmark.AverageDuration("Total").Microseconds()}({Benchmark.MaxDuration("Total").Microseconds()})µs");
        }
        
        private void DrawProximalPositionGizmos()
        {
            int regionSize = currentUniverse.RegionGenerationSettings.Size;
            int regionChunkSize = currentUniverse.RegionGenerationSettings.ChunkSize;
            int sectorSize = currentUniverse.SectorGenerationSettings.Size;
            int sectorChunkSize = currentUniverse.SectorGenerationSettings.ChunkSize;
            int filamentSize = currentUniverse.FilamentGenerationSettings.Size;
            int filamentChunkSize = currentUniverse.FilamentGenerationSettings.ChunkSize;

            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(transform.position);
            Universe.Region.Position currentRegionPosition = currentRegionChunkPosition.RegionPosition;
            Universe.Sector.Chunk.Position currentSectorChunkPosition = new Universe.Sector.Chunk.Position(transform.position);
            Universe.Sector.Position currentSectorPosition = currentSectorChunkPosition.SectorPosition;
            Universe.Filament.Chunk.Position currentFilamentChunkPosition = new Universe.Filament.Chunk.Position(transform.position);
            Universe.Filament.Position currentFilamentPosition = currentFilamentChunkPosition.FilamentPosition;

            /*
            UnityEngine.Debug.Log($"Region Chunk Pos: {currentRegionChunkPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Region Pos: {currentRegionPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Sector Chunk Pos: {currentSectorChunkPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Sector Pos: {currentSectorPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Filament Chunk Pos: {currentFilamentChunkPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Filament Pos: {currentFilamentPosition.VectorIntPosition}");
            */

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
