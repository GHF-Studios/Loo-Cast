using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Observer
{
    using Game;
    using System.Linq;
    using Universe;

    public class UniverseObserver : MonoBehaviour
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
        private int sectorChunkLoadRadius = 1;
        private int filamentChunkLoadRadius = 1;
        private Universe currentUniverse;

        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
        }

        private void Update()
        {
            regionChunkScreenRadius = (int)Vector3.Distance(transform.position, Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            Universe.Region.Chunk.Position currentRegionChunkPosition = new Universe.Region.Chunk.Position(currentUniverse, transform.position);
            GetProximalPositions(currentRegionChunkPosition, totalRegionChunkLoadRadius, out List<Universe.Region.Chunk.Position> regionChunkPositions, out List<Universe.Sector.Chunk.Position> sectorChunkPositions, out List<Universe.Filament.Chunk.Position> filamentChunkPositions, out List<Universe.Region.Position> regionPositions, out List<Universe.Sector.Position> sectorPositions, out List<Universe.Filament.Position> filamentPositions);

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
        }

        private void GetProximalPositions(Universe.Region.Chunk.Position regionChunkCenterPosition, int regionChunkRadius, out List<Universe.Region.Chunk.Position> regionChunkPositions, out List<Universe.Sector.Chunk.Position> sectorChunkPositions, out List<Universe.Filament.Chunk.Position> filamentChunkPositions, out List<Universe.Region.Position> regionPositions, out List<Universe.Sector.Position> sectorPositions, out List<Universe.Filament.Position> filamentPositions)
        {
            regionChunkPositions = new List<Universe.Region.Chunk.Position>();
            sectorChunkPositions = new List<Universe.Sector.Chunk.Position>();
            filamentChunkPositions = new List<Universe.Filament.Chunk.Position>();
            regionPositions = new List<Universe.Region.Position>();
            sectorPositions = new List<Universe.Sector.Position>();
            filamentPositions = new List<Universe.Filament.Position>();
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
