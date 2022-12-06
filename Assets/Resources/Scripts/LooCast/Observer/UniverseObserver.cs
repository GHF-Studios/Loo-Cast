using System;
using System.Collections.Generic;
using System.Diagnostics;
using UnityEngine;

namespace LooCast.Observer
{
    using Core;
    using Game;
    using Universe;

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

        private long maxTotalCalculationTime = 0;
        private long maxAverageRowCalculationTime = 0;
        private long maxRegionChunkCalculationTime = 0;
        private long maxSectorChunkCalculationTime = 0;
        private long maxFilamentChunkCalculationTime = 0;
        private long maxRegionCalculationTime = 0;
        private long maxSectorCalculationTime = 0;
        private long maxFilamentCalculationTime = 0;
        private long maxRowElementCalculationTime = 0;

        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
            currentRegionPositionOffset = new Universe.Region.Position(currentUniverse, Vector2Int.zero);
            Stopwatch stopwatch = new Stopwatch();

            UpdateProximalPositions();

            /*
            stopwatch.Start();
            #region Filament Loading
            foreach (Universe.Filament.Position filamentPosition in filamentPositions)
            {
                if (!currentUniverse.IsFilamentGenerated(filamentPosition))
                {
                    currentUniverse.GenerateFilament(filamentPosition);
                }
                else if (!currentUniverse.IsFilamentLoaded(filamentPosition))
                {
                    currentUniverse.LoadFilament(filamentPosition);
                }
            }

            foreach (Universe.Filament.Chunk.Position filamentChunkPosition in filamentChunkPositions)
            {
                if (!currentUniverse.IsFilamentChunkGenerated(filamentChunkPosition))
                {
                    currentUniverse.GenerateFilamentChunk(filamentChunkPosition);
                }
                else if (!currentUniverse.IsFilamentChunkLoaded(filamentChunkPosition))
                {
                    currentUniverse.LoadFilamentChunk(filamentChunkPosition);
                }
            }
            #endregion
            stopwatch.Stop();
            UnityEngine.Debug.Log($"[UniverseObserver] Took {stopwatch.ElapsedMilliseconds}ms to Load Filaments!");

            stopwatch.Restart();
            #region Sector Loading
            foreach (Universe.Sector.Position sectorPosition in sectorPositions)
            {
                if (!currentUniverse.IsSectorGenerated(sectorPosition))
                {
                    currentUniverse.GenerateSector(sectorPosition);
                }
                else if (!currentUniverse.IsSectorLoaded(sectorPosition))
                {
                    currentUniverse.LoadSector(sectorPosition);
                }
            }

            foreach (Universe.Sector.Chunk.Position sectorChunkPosition in sectorChunkPositions)
            {
                if (!currentUniverse.IsSectorChunkGenerated(sectorChunkPosition))
                {
                    currentUniverse.GenerateSectorChunk(sectorChunkPosition);
                }
                else if (!currentUniverse.IsSectorChunkLoaded(sectorChunkPosition))
                {
                    currentUniverse.LoadSectorChunk(sectorChunkPosition);
                }
            }
            #endregion
            stopwatch.Stop();
            UnityEngine.Debug.Log($"[UniverseObserver] Took {stopwatch.ElapsedMilliseconds}ms to Load Sectors!");

            stopwatch.Restart();
            #region Region Loading
            foreach (Universe.Region.Position regionPosition in regionPositions)
            {
                if (!currentUniverse.IsRegionGenerated(regionPosition))
                {
                    currentUniverse.GenerateRegion(regionPosition);
                }
                else if (!currentUniverse.IsRegionLoaded(regionPosition))
                {
                    currentUniverse.LoadRegion(regionPosition);
                }
            }

            foreach (Universe.Region.Chunk.Position regionChunkPosition in regionChunkPositions)
            {
                if (!currentUniverse.IsRegionChunkGenerated(regionChunkPosition))
                {
                    currentUniverse.GenerateRegionChunk(regionChunkPosition);
                }
                else if (!currentUniverse.IsRegionChunkLoaded(regionChunkPosition))
                {
                    currentUniverse.LoadRegionChunk(regionChunkPosition);
                }
            }
            #endregion
            stopwatch.Stop();
            UnityEngine.Debug.Log($"[UniverseObserver] Took {stopwatch.ElapsedMilliseconds}ms to Load Regions!");
            */
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
                currentRegionPositionOffset = new Universe.Region.Position(currentUniverse, currentRegionPositionOffset.VectorIntPosition + currentRegionPositionDifference);
                transform.Translate(-new Vector3(currentRegionPositionDifference.x * regionSize, currentRegionPositionDifference.y * regionSize));
            }
        }

        private void GetProximalPositions(out List<Universe.Region.Chunk.Position> regionChunkPositions, out List<Universe.Sector.Chunk.Position> sectorChunkPositions, out List<Universe.Filament.Chunk.Position> filamentChunkPositions, out List<Universe.Region.Position> regionPositions, out List<Universe.Sector.Position> sectorPositions, out List<Universe.Filament.Position> filamentPositions)
        {
            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, transform.position);

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

            Universe.Region.Chunk.Position screenRegionChunkPosMin = new Universe.Region.Chunk.Position(currentUniverse, (Vector2)Camera.main.ScreenToWorldPoint(new Vector2(0, 0)));
            screenRegionChunkPosMin = new Universe.Region.Chunk.Position(currentUniverse, screenRegionChunkPosMin.VectorIntPosition - (Vector2Int.one * regionChunkLoadRadius));
            Universe.Region.Chunk.Position screenRegionChunkPosMax = new Universe.Region.Chunk.Position(currentUniverse, (Vector2)Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            screenRegionChunkPosMax = new Universe.Region.Chunk.Position(currentUniverse, screenRegionChunkPosMax.VectorIntPosition + (Vector2Int.one * regionChunkLoadRadius));

            int calculatedRows = 0;
            long totalCalculationTime = 0;
            long rowCalculationTime = 0;
            long averageRowCalculationTime = 0;
            long regionChunkCalculationTime = 0;
            long sectorChunkCalculationTime = 0;
            long filamentChunkCalculationTime = 0;
            long regionCalculationTime = 0;
            long sectorCalculationTime = 0;
            long filamentCalculationTime = 0;
            long rowElementCalculationTime = 0;
            Stopwatch totalRowStopwatch = new Stopwatch();
            Stopwatch rowStopwatch = new Stopwatch();
            Stopwatch utilityStopwatch = new Stopwatch();

            totalRowStopwatch.Start();
            for (int x = screenRegionChunkPosMin.VectorIntPosition.x; x <= screenRegionChunkPosMax.VectorIntPosition.x; x++)
            {
                rowStopwatch.Restart();
                for (int y = screenRegionChunkPosMin.VectorIntPosition.y; y <= screenRegionChunkPosMax.VectorIntPosition.y; y++)
                {
                    utilityStopwatch.Restart();
                    Universe.Region.Chunk.Position regionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, new Vector2Int(x, y));
                    if (!regionChunkPositions.Contains(regionChunkPosition))
                    {
                        regionChunkPositions.Add(regionChunkPosition);
                    }
                    utilityStopwatch.Stop();
                    regionChunkCalculationTime += utilityStopwatch.ElapsedMilliseconds;
                    rowElementCalculationTime += utilityStopwatch.ElapsedMilliseconds;

                    utilityStopwatch.Restart();
                    Universe.Sector.Chunk.Position sectorChunkPosition = new Universe.Sector.Chunk.Position(currentUniverse, regionChunkPosition.WorldPosition);
                    if (!sectorChunkPositions.Contains(sectorChunkPosition))
                    {
                        sectorChunkPositions.Add(sectorChunkPosition);
                    }
                    utilityStopwatch.Stop();
                    sectorChunkCalculationTime += utilityStopwatch.ElapsedMilliseconds;
                    rowElementCalculationTime += utilityStopwatch.ElapsedMilliseconds;

                    utilityStopwatch.Restart();
                    Universe.Filament.Chunk.Position filamentChunkPosition = new Universe.Filament.Chunk.Position(currentUniverse, regionChunkPosition.WorldPosition);
                    if (!filamentChunkPositions.Contains(filamentChunkPosition))
                    {
                        filamentChunkPositions.Add(filamentChunkPosition);
                    }
                    utilityStopwatch.Stop();
                    filamentChunkCalculationTime += utilityStopwatch.ElapsedMilliseconds;
                    rowElementCalculationTime += utilityStopwatch.ElapsedMilliseconds;

                    utilityStopwatch.Restart();
                    Universe.Region.Position regionPosition = new Universe.Region.Position(currentUniverse, regionChunkPosition.WorldPosition);
                    if (!regionPositions.Contains(regionPosition))
                    {
                        regionPositions.Add(regionPosition);
                    }
                    utilityStopwatch.Stop();
                    regionCalculationTime += utilityStopwatch.ElapsedMilliseconds;
                    rowElementCalculationTime += utilityStopwatch.ElapsedMilliseconds;

                    utilityStopwatch.Restart();
                    Universe.Sector.Position sectorPosition = new Universe.Sector.Position(currentUniverse, regionChunkPosition.WorldPosition);
                    if (!sectorPositions.Contains(sectorPosition))
                    {
                        sectorPositions.Add(sectorPosition);
                    }
                    utilityStopwatch.Stop();
                    sectorCalculationTime += utilityStopwatch.ElapsedMilliseconds;
                    rowElementCalculationTime += utilityStopwatch.ElapsedMilliseconds;

                    utilityStopwatch.Restart();
                    Universe.Filament.Position filamentPosition = new Universe.Filament.Position(currentUniverse, regionChunkPosition.WorldPosition);
                    if (!filamentPositions.Contains(filamentPosition))
                    {
                        filamentPositions.Add(filamentPosition);
                    }
                    utilityStopwatch.Stop();
                    filamentCalculationTime += utilityStopwatch.ElapsedMilliseconds;
                    rowElementCalculationTime += utilityStopwatch.ElapsedMilliseconds;

                    if (maxAverageRowCalculationTime < rowCalculationTime)
                    {
                        maxAverageRowCalculationTime = rowCalculationTime;
                    }
                    if (maxRegionChunkCalculationTime < regionChunkCalculationTime)
                    {
                        maxRegionChunkCalculationTime = regionChunkCalculationTime;
                    }
                    if (maxSectorChunkCalculationTime < sectorChunkCalculationTime)
                    {
                        maxSectorChunkCalculationTime = sectorChunkCalculationTime;
                    }
                    if (maxFilamentChunkCalculationTime < filamentChunkCalculationTime)
                    {
                        maxFilamentChunkCalculationTime = filamentChunkCalculationTime;
                    }
                    if (maxRegionCalculationTime < regionCalculationTime)
                    {
                        maxRegionCalculationTime = regionCalculationTime;
                    }
                    if (maxSectorCalculationTime < sectorCalculationTime)
                    {
                        maxSectorCalculationTime = sectorCalculationTime;
                    }
                    if (maxFilamentCalculationTime < filamentCalculationTime)
                    {
                        maxFilamentCalculationTime = filamentCalculationTime;
                    }
                    if (maxRowElementCalculationTime < rowElementCalculationTime)
                    {
                        maxRowElementCalculationTime = rowElementCalculationTime;
                    }
                }
                rowStopwatch.Stop();
                calculatedRows += 1;
                rowCalculationTime += rowStopwatch.ElapsedMilliseconds;
            }
            totalRowStopwatch.Stop();
            totalCalculationTime = totalRowStopwatch.ElapsedMilliseconds;
            averageRowCalculationTime = rowCalculationTime / calculatedRows;
            if (maxTotalCalculationTime < totalCalculationTime)
            {
                maxTotalCalculationTime = totalCalculationTime;
            }
            if (maxAverageRowCalculationTime < averageRowCalculationTime)
            {
                maxAverageRowCalculationTime = averageRowCalculationTime;
            }
            UnityEngine.Debug.Log($"\t\t\t\tRegion: \t{maxRegionCalculationTime}({regionCalculationTime})ms\t\t\tChunk: \t{maxRegionChunkCalculationTime}({regionChunkCalculationTime})ms");
            UnityEngine.Debug.Log($"\t\t\t\tSector: \t{maxSectorCalculationTime}({sectorCalculationTime})ms\t\t\tChunk: \t{maxSectorChunkCalculationTime}({sectorChunkCalculationTime})ms");
            UnityEngine.Debug.Log($"\t\t\t\tFilament: {maxFilamentCalculationTime}({filamentCalculationTime})ms\t\tChunk: \t{maxFilamentChunkCalculationTime}({filamentChunkCalculationTime})ms");
            UnityEngine.Debug.Log($"\t\t\t\tElement: {maxRowElementCalculationTime}({rowElementCalculationTime})ms\t\tRow: \t{maxAverageRowCalculationTime}({averageRowCalculationTime})ms\t\t Total: \t{maxTotalCalculationTime}({totalCalculationTime})ms");
        }
        
        private void DrawProximalPositionGizmos()
        {
            int regionSize = currentUniverse.RegionGenerationSettings.Size;
            int regionChunkSize = currentUniverse.RegionGenerationSettings.ChunkSize;
            int sectorSize = currentUniverse.SectorGenerationSettings.Size;
            int sectorChunkSize = currentUniverse.SectorGenerationSettings.ChunkSize;
            int filamentSize = currentUniverse.FilamentGenerationSettings.Size;
            int filamentChunkSize = currentUniverse.FilamentGenerationSettings.ChunkSize;

            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, transform.position);
            Universe.Region.Position currentRegionPosition = currentRegionChunkPosition.RegionPosition;
            Universe.Sector.Chunk.Position currentSectorChunkPosition = new Universe.Sector.Chunk.Position(currentUniverse, transform.position);
            Universe.Sector.Position currentSectorPosition = currentSectorChunkPosition.SectorPosition;
            Universe.Filament.Chunk.Position currentFilamentChunkPosition = new Universe.Filament.Chunk.Position(currentUniverse, transform.position);
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
