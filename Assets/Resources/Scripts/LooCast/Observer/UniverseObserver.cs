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
            Vector2Int currentRegionChunkPosition = Universe.Region.Chunk.GetChunkPosition(currentUniverse, transform.position);
            GetProximalPositions(currentRegionChunkPosition, totalRegionChunkLoadRadius, out List<Vector2Int> regionChunkPositions, out List<Vector2Int> sectorChunkPositions, out List<Vector2Int> filamentChunkPositions, out List<Vector2Int> regionPositions, out List<Vector2Int> sectorPositions, out List<Vector2Int> filamentPositions);

            #region Filament Loading
            foreach (Vector2Int filamentPosition in filamentPositions)
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

            foreach (Vector2Int filamentChunkPosition in filamentChunkPositions)
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
            foreach (Vector2Int sectorPosition in sectorPositions)
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

            foreach (Vector2Int sectorChunkPosition in sectorChunkPositions)
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
            foreach (Vector2Int regionPosition in regionPositions)
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

            foreach (Vector2Int regionChunkPosition in regionChunkPositions)
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

        private void GetProximalPositions(Vector2Int regionChunkCenterPosition, int regionChunkRadius, out List<Vector2Int> regionChunkPositions, out List<Vector2Int> sectorChunkPositions, out List<Vector2Int> filamentChunkPositions, out List<Vector2Int> regionPositions, out List<Vector2Int> sectorPositions, out List<Vector2Int> filamentPositions)
        {
            regionChunkPositions = new List<Vector2Int>();
            sectorChunkPositions = new List<Vector2Int>();
            filamentChunkPositions = new List<Vector2Int>();
            regionPositions = new List<Vector2Int>();
            sectorPositions = new List<Vector2Int>();
            filamentPositions = new List<Vector2Int>();
            Vector2Int regionChunkPositionMin = new Vector2Int(regionChunkCenterPosition.x - regionChunkRadius, regionChunkCenterPosition.y - regionChunkRadius);
            Vector2Int regionChunkPositionMax = new Vector2Int(regionChunkCenterPosition.x + regionChunkRadius, regionChunkCenterPosition.y + regionChunkRadius);
            for (int x = regionChunkPositionMin.x; x <= regionChunkPositionMax.x; x++)
            {
                for (int y = regionChunkPositionMin.y; y < regionChunkPositionMax.y; y++)
                {
                    Vector2Int regionChunkPosition = new Vector2Int(x, y);
                    if (Vector2Int.Distance(regionChunkCenterPosition, regionChunkPosition) <= regionChunkRadius)
                    {
                        if (!regionChunkPositions.Contains(regionChunkPosition))
                        {
                            regionChunkPositions.Add(regionChunkPosition);
                        }

                        Vector2Int sectorChunkPosition = Universe.Sector.Chunk.GetChunkPosition(currentUniverse, transform.position);
                        if (!sectorChunkPositions.Contains(sectorChunkPosition))
                        {
                            sectorChunkPositions.Add(sectorChunkPosition);
                        }

                        Vector2Int filamentChunkPosition = Universe.Filament.Chunk.GetChunkPosition(currentUniverse, transform.position);
                        if (!filamentChunkPositions.Contains(filamentChunkPosition))
                        {
                            filamentChunkPositions.Add(filamentChunkPosition);
                        }

                        Vector2Int regionPosition = Universe.Region.GetRegionPosition(currentUniverse, transform.position);
                        if (!regionPositions.Contains(regionPosition))
                        {
                            regionPositions.Add(regionPosition);
                        }

                        Vector2Int sectorPosition = Universe.Sector.GetSectorPosition(currentUniverse, transform.position);
                        if (!sectorPositions.Contains(sectorPosition))
                        {
                            sectorPositions.Add(sectorPosition);
                        }

                        Vector2Int filamentPosition = Universe.Filament.GetFilamentPosition(currentUniverse, transform.position);
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
