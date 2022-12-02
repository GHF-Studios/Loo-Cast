﻿using System;
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
        

        private int totalRegionChunkLoadRadius
        {
            get
            {
                return regionChunkLoadRadius + regionChunkScreenRadius;
            }
        }

        private int regionChunkScreenRadius;
        private int regionChunkLoadRadius = 32;
        private Universe currentUniverse;

        private Universe.Region.Position currentRegionPosition;
        private Universe.Region.Position currentRegionPositionOffset;

        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;

            currentRegionPosition = new Universe.Region.Position(currentUniverse, transform.position);
            currentRegionPositionOffset = new Universe.Region.Position(currentUniverse, Vector2Int.zero);

            Stopwatch stopwatch = new Stopwatch();

            regionChunkScreenRadius = (int)Vector3.Distance(transform.position, Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, transform.position);
            GetProximalPositions(currentRegionChunkPosition, totalRegionChunkLoadRadius, out List<Universe.Region.Chunk.Position> regionChunkPositions, out List<Universe.Sector.Chunk.Position> sectorChunkPositions, out List<Universe.Filament.Chunk.Position> filamentChunkPositions, out List<Universe.Region.Position> regionPositions, out List<Universe.Sector.Position> sectorPositions, out List<Universe.Filament.Position> filamentPositions);

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
        }

        private void Update()
        {
            currentRegionPosition = new Universe.Region.Position(currentUniverse, transform.position);
            // TranslateRegions(currentRegionPosition.VectorIntPosition);
        }

        private void OnDrawGizmos()
        {
            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, transform.position);
            Universe.Region.Position currentRegionPosition = currentRegionChunkPosition.RegionPosition;
            Universe.Sector.Chunk.Position currentSectorChunkPosition = new Universe.Sector.Chunk.Position(currentUniverse, transform.position);
            Universe.Sector.Position currentSectorPosition = currentSectorChunkPosition.SectorPosition;
            Universe.Filament.Chunk.Position currentFilamentChunkPosition = new Universe.Filament.Chunk.Position(currentUniverse, transform.position);
            Universe.Filament.Position currentFilamentPosition = currentFilamentChunkPosition.FilamentPosition;
            int regionSize = currentUniverse.RegionGenerationSettings.Size;
            int regionChunkSize = currentUniverse.RegionGenerationSettings.ChunkSize;
            int sectorSize = currentUniverse.SectorGenerationSettings.Size;
            int sectorChunkSize = currentUniverse.SectorGenerationSettings.ChunkSize;
            int filamentSize = currentUniverse.FilamentGenerationSettings.Size;
            int filamentChunkSize = currentUniverse.FilamentGenerationSettings.ChunkSize;

            UnityEngine.Debug.Log($"Region Chunk Pos: {currentRegionChunkPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Region Pos: {currentRegionPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Sector Chunk Pos: {currentSectorChunkPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Sector Pos: {currentSectorPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Filament Chunk Pos: {currentFilamentChunkPosition.VectorIntPosition}");
            UnityEngine.Debug.Log($"Filament Pos: {currentFilamentPosition.VectorIntPosition}");

            // Region Chunk
            Gizmos.color = Color.green;
            Gizmos.DrawWireCube(currentRegionChunkPosition.WorldPosition, new Vector2(regionChunkSize, regionChunkSize));

            // Region
            Gizmos.color = Color.red;
            Gizmos.DrawWireCube(currentRegionPosition.WorldPosition, new Vector2(regionSize, regionSize));

            // Sector Chunk
            Gizmos.color = Color.yellow;
            Gizmos.DrawWireCube(currentSectorChunkPosition.WorldPosition, new Vector2(regionSize * sectorChunkSize, regionSize * sectorChunkSize));

            // Sector
            Gizmos.color = Color.blue;
            Gizmos.DrawWireCube(currentSectorPosition.WorldPosition, new Vector2(regionSize * sectorSize, regionSize * sectorSize));

            // Filament Chunk
            Gizmos.color = new Color(1.0f, 0.0f, 1.0f);
            Gizmos.DrawWireCube(currentFilamentChunkPosition.WorldPosition, new Vector2(regionSize * sectorSize * filamentChunkSize, regionSize * sectorSize * filamentChunkSize));

            // Filament
            Gizmos.color = new Color(1.0f, 0.5f, 0.0f);
            Gizmos.DrawWireCube(currentFilamentPosition.WorldPosition, new Vector2(regionSize * sectorSize * filamentSize, regionSize * sectorSize * filamentSize));
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

        private void GetProximalPositions(Universe.Region.Chunk.Position regionChunkCenterPosition, int regionChunkRadius, out List<Universe.Region.Chunk.Position> regionChunkPositions, out List<Universe.Sector.Chunk.Position> sectorChunkPositions, out List<Universe.Filament.Chunk.Position> filamentChunkPositions, out List<Universe.Region.Position> regionPositions, out List<Universe.Sector.Position> sectorPositions, out List<Universe.Filament.Position> filamentPositions)
        {
            regionChunkPositions = new List<Universe.Region.Chunk.Position>();
            sectorChunkPositions = new List<Universe.Sector.Chunk.Position>();
            filamentChunkPositions = new List<Universe.Filament.Chunk.Position>();
            regionPositions = new List<Universe.Region.Position>();
            sectorPositions = new List<Universe.Sector.Position>();
            filamentPositions = new List<Universe.Filament.Position>();

            #region DEV
            regionChunkPositions.Add(regionChunkCenterPosition);
            sectorChunkPositions.Add(new Universe.Sector.Chunk.Position(currentUniverse, regionChunkCenterPosition.WorldPosition));
            filamentChunkPositions.Add(new Universe.Filament.Chunk.Position(currentUniverse, regionChunkCenterPosition.WorldPosition));
            regionPositions.Add(regionChunkCenterPosition.RegionPosition);
            sectorPositions.Add(new Universe.Sector.Position(currentUniverse, regionChunkCenterPosition.WorldPosition));
            filamentPositions.Add(new Universe.Filament.Position(currentUniverse, regionChunkCenterPosition.WorldPosition));

            return;
            #endregion

            Vector2Int regionChunkPositionMin = new Vector2Int(regionChunkCenterPosition.VectorIntPosition.x - regionChunkRadius, regionChunkCenterPosition.VectorIntPosition.y - regionChunkRadius);
            Vector2Int regionChunkPositionMax = new Vector2Int(regionChunkCenterPosition.VectorIntPosition.x + regionChunkRadius, regionChunkCenterPosition.VectorIntPosition.y + regionChunkRadius);
            for (int x = regionChunkPositionMin.x; x <= regionChunkPositionMax.x; x++)
            {
                for (int y = regionChunkPositionMin.y; y < regionChunkPositionMax.y; y++)
                {
                    Universe.Region.Chunk.Position regionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, new Vector2(x, y));
                    if (Vector2Int.Distance(regionChunkCenterPosition.VectorIntPosition, regionChunkPosition.VectorIntPosition) <= regionChunkRadius)
                    {
                        if (!regionChunkPositions.Contains(regionChunkPosition))
                        {
                            regionChunkPositions.Add(regionChunkPosition);
                        }

                        Universe.Sector.Chunk.Position sectorChunkPosition = new Universe.Sector.Chunk.Position(currentUniverse, transform.position);
                        if (!sectorChunkPositions.Contains(sectorChunkPosition))
                        {
                            sectorChunkPositions.Add(sectorChunkPosition);
                        }

                        Universe.Filament.Chunk.Position filamentChunkPosition = new Universe.Filament.Chunk.Position(currentUniverse, transform.position);
                        if (!filamentChunkPositions.Contains(filamentChunkPosition))
                        {
                            filamentChunkPositions.Add(filamentChunkPosition);
                        }

                        Universe.Region.Position regionPosition = new Universe.Region.Position(currentUniverse, transform.position);
                        if (!regionPositions.Contains(regionPosition))
                        {
                            regionPositions.Add(regionPosition);
                        }

                        Universe.Sector.Position sectorPosition = new Universe.Sector.Position(currentUniverse, transform.position);
                        if (!sectorPositions.Contains(sectorPosition))
                        {
                            sectorPositions.Add(sectorPosition);
                        }

                        Universe.Filament.Position filamentPosition = new Universe.Filament.Position(currentUniverse, transform.position);
                        if (!filamentPositions.Contains(filamentPosition))
                        {
                            filamentPositions.Add(filamentPosition);
                        }
                    }
                }
            }
        }
    }
}
