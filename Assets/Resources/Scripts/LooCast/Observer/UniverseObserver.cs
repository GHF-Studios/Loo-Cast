using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Observer
{
    using Core;
    using Game;
    using Universe;
    using Diagnostic;
    using Util;
    using System.Threading.Tasks;
    using System.Collections.Concurrent;

    // ISSUE TRACKER
    // 1. Loaded Chunks never get unloaded

    public class UniverseObserver : ExtendedMonoBehaviour
    {
        private Universe currentUniverse;
        private int regionChunkLoadRadius;

        private Universe.Region.Chunk.Position previousScreenRegionChunkPosMin;
        private Universe.Region.Chunk.Position previousScreenRegionChunkPosMax;
        private Universe.Region.Chunk.Position currentScreenRegionChunkPosMin;
        private Universe.Region.Chunk.Position currentScreenRegionChunkPosMax;

        private Universe.Sector.Chunk.Position previousScreenSectorChunkPosMin;
        private Universe.Sector.Chunk.Position previousScreenSectorChunkPosMax;
        private Universe.Sector.Chunk.Position currentScreenSectorChunkPosMin;
        private Universe.Sector.Chunk.Position currentScreenSectorChunkPosMax;

        private Universe.Filament.Chunk.Position previousScreenFilamentChunkPosMin;
        private Universe.Filament.Chunk.Position previousScreenFilamentChunkPosMax;
        private Universe.Filament.Chunk.Position currentScreenFilamentChunkPosMin;
        private Universe.Filament.Chunk.Position currentScreenFilamentChunkPosMax;

        private Universe.Region.Position previousScreenRegionPosMin;
        private Universe.Region.Position previousScreenRegionPosMax;
        private Universe.Region.Position currentScreenRegionPosMin;
        private Universe.Region.Position currentScreenRegionPosMax;

        private Universe.Sector.Position previousScreenSectorPosMin;
        private Universe.Sector.Position previousScreenSectorPosMax;
        private Universe.Sector.Position currentScreenSectorPosMin;
        private Universe.Sector.Position currentScreenSectorPosMax;

        private Universe.Filament.Position previousScreenFilamentPosMin;
        private Universe.Filament.Position previousScreenFilamentPosMax;
        private Universe.Filament.Position currentScreenFilamentPosMin;
        private Universe.Filament.Position currentScreenFilamentPosMax;

        private ConcurrentBag<Universe.Region.Chunk.Position> previouslyProximalRegionChunkPositions;
        private ConcurrentBag<Universe.Sector.Chunk.Position> previouslyProximalSectorChunkPositions;
        private ConcurrentBag<Universe.Filament.Chunk.Position> previouslyProximalFilamentChunkPositions;
        private ConcurrentBag<Universe.Region.Position> previouslyProximalRegionPositions;
        private ConcurrentBag<Universe.Sector.Position> previouslyProximalSectorPositions;
        private ConcurrentBag<Universe.Filament.Position> previouslyProximalFilamentPositions;

        private ConcurrentBag<Universe.Region.Chunk.Position> newlyProximalRegionChunkPositions;
        private ConcurrentBag<Universe.Sector.Chunk.Position> newlyProximalSectorChunkPositions;
        private ConcurrentBag<Universe.Filament.Chunk.Position> newlyProximalFilamentChunkPositions;
        private ConcurrentBag<Universe.Region.Position> newlyProximalRegionPositions;
        private ConcurrentBag<Universe.Sector.Position> newlyProximalSectorPositions;
        private ConcurrentBag<Universe.Filament.Position> newlyProximalFilamentPositions;

        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
            regionChunkLoadRadius = 2;

            Benchmark.Create("UpdatePositions");
            //Benchmark.Create("UpdatePosition");
            Benchmark.Create("LoadPositions");
            Benchmark.Create("LoadRegion");
            Benchmark.Create("LoadSector");
            Benchmark.Create("LoadFilament");
            Benchmark.Create("LoadRegionChunk");
            Benchmark.Create("LoadSectorChunk");
            Benchmark.Create("LoadFilamentChunk");
            Benchmark.Create("UnloadPositions");

            
            InitializeScreenPositions();
            InitializeProximalPositions();
            LoadNewlyProximalPositions();
        }

        private void Update()
        {
            UpdateScreenPositions();
            UpdateProximalPositions();
            UnloadPreviouslyProximalPositions();
            LoadNewlyProximalPositions();
            PrintBenchmarks();
        }

        private void OnDrawGizmos()
        {
            DrawLoadedPositionGizmos();
        }

        private void InitializeScreenPositions()
        {
            UpdateScreenPositions();
            
            if (previousScreenRegionChunkPosMin == null || previousScreenRegionChunkPosMax == null)
            {
                previousScreenRegionChunkPosMin = currentScreenRegionChunkPosMin;
                previousScreenRegionChunkPosMax = currentScreenRegionChunkPosMax;
            }

            if (previousScreenSectorChunkPosMin == null || previousScreenSectorChunkPosMax == null)
            {
                previousScreenSectorChunkPosMin = currentScreenSectorChunkPosMin;
                previousScreenSectorChunkPosMax = currentScreenSectorChunkPosMax;
            }

            if (previousScreenFilamentChunkPosMin == null || previousScreenFilamentChunkPosMax == null)
            {
                previousScreenFilamentChunkPosMin = currentScreenFilamentChunkPosMin;
                previousScreenFilamentChunkPosMax = currentScreenFilamentChunkPosMax;
            }

            if (previousScreenRegionPosMin == null || previousScreenRegionPosMax == null)
            {
                previousScreenRegionPosMin = currentScreenRegionPosMin;
                previousScreenRegionPosMax = currentScreenRegionPosMax;
            }

            if (previousScreenSectorPosMin == null || previousScreenSectorPosMax == null)
            {
                previousScreenSectorPosMin = currentScreenSectorPosMin;
                previousScreenSectorPosMax = currentScreenSectorPosMax;
            }

            if (previousScreenFilamentPosMin == null || previousScreenFilamentPosMax == null)
            {
                previousScreenFilamentPosMin = currentScreenFilamentPosMin;
                previousScreenFilamentPosMax = currentScreenFilamentPosMax;
            }
        }

        private void InitializeProximalPositions()
        {
            previouslyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();
            previouslyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();
            previouslyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();
            previouslyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();
            previouslyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();
            previouslyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();
            
            newlyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();
            newlyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();
            newlyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();
            newlyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();
            newlyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();
            newlyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();

            Parallel.For(previousScreenRegionChunkPosMin.CurrentPosition.x, previousScreenRegionChunkPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenRegionChunkPosMin.CurrentPosition.y, previousScreenRegionChunkPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalRegionChunkPositions.Add(new Universe.Region.Chunk.Position(new Vector2Int(x, y)));
                });
            });

            Parallel.For(previousScreenSectorChunkPosMin.CurrentPosition.x, previousScreenSectorChunkPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenSectorChunkPosMin.CurrentPosition.y, previousScreenSectorChunkPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(x, y)));
                });
            });

            Parallel.For(previousScreenFilamentChunkPosMin.CurrentPosition.x, previousScreenFilamentChunkPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenFilamentChunkPosMin.CurrentPosition.y, previousScreenFilamentChunkPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(x, y)));
                });
            });

            Parallel.For(previousScreenRegionPosMin.CurrentPosition.x, previousScreenRegionPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenRegionPosMin.CurrentPosition.y, previousScreenRegionPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(x, y)));
                });
            });

            Parallel.For(previousScreenSectorPosMin.CurrentPosition.x, previousScreenSectorPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenSectorPosMin.CurrentPosition.y, previousScreenSectorPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(x, y)));
                });
            });

            Parallel.For(previousScreenFilamentPosMin.CurrentPosition.x, previousScreenFilamentPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenFilamentPosMin.CurrentPosition.y, previousScreenFilamentPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(x, y)));
                });
            });
        }

        private void UpdateScreenPositions()
        {
            currentScreenRegionChunkPosMin = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(0, 0)));
            currentScreenRegionChunkPosMin = new Universe.Region.Chunk.Position(currentScreenRegionChunkPosMin.CurrentPosition - (Vector2Int.one * regionChunkLoadRadius));
            currentScreenRegionChunkPosMax = new Universe.Region.Chunk.Position((Vector2)Camera.main.ScreenToWorldPoint(new Vector2(Screen.width - 1, Screen.height - 1)));
            currentScreenRegionChunkPosMax = new Universe.Region.Chunk.Position(currentScreenRegionChunkPosMax.CurrentPosition + (Vector2Int.one * regionChunkLoadRadius));

            currentScreenSectorChunkPosMin = currentScreenRegionChunkPosMin.SectorChunkPosition;
            currentScreenSectorChunkPosMax = currentScreenRegionChunkPosMax.SectorChunkPosition;

            currentScreenFilamentChunkPosMin = currentScreenRegionChunkPosMin.FilamentChunkPosition;
            currentScreenFilamentChunkPosMax = currentScreenRegionChunkPosMax.FilamentChunkPosition;

            currentScreenRegionPosMin = currentScreenRegionChunkPosMin.RegionPosition;
            currentScreenRegionPosMax = currentScreenRegionChunkPosMax.RegionPosition;

            currentScreenSectorPosMin = currentScreenSectorChunkPosMin.SectorPosition;
            currentScreenSectorPosMax = currentScreenSectorChunkPosMax.SectorPosition;

            currentScreenFilamentPosMin = currentScreenFilamentChunkPosMin.FilamentPosition;
            currentScreenFilamentPosMax = currentScreenFilamentChunkPosMax.FilamentPosition;
        }

        private void UpdateProximalPositions()
        {
            #region Calculate Screen Position Offsets
            Vector2Int screenRegionChunkPosMinOffset = Vector2Int.zero;
            Vector2Int screenRegionChunkPosMaxOffset = Vector2Int.zero;
            if (currentScreenRegionChunkPosMin != previousScreenRegionChunkPosMin)
            {
                screenRegionChunkPosMinOffset = currentScreenRegionChunkPosMin.CurrentPosition - previousScreenRegionChunkPosMin.CurrentPosition;
            }
            if (currentScreenRegionChunkPosMax != previousScreenRegionChunkPosMax)
            {
                screenRegionChunkPosMaxOffset = currentScreenRegionChunkPosMax.CurrentPosition - previousScreenRegionChunkPosMax.CurrentPosition;
            }

            Vector2Int screenSectorChunkPosMinOffset = Vector2Int.zero;
            Vector2Int screenSectorChunkPosMaxOffset = Vector2Int.zero;
            if (currentScreenSectorChunkPosMin != previousScreenSectorChunkPosMin)
            {
                screenSectorChunkPosMinOffset = currentScreenSectorChunkPosMin.CurrentPosition - previousScreenSectorChunkPosMin.CurrentPosition;
            }
            if (currentScreenSectorChunkPosMax != previousScreenSectorChunkPosMax)
            {
                screenSectorChunkPosMaxOffset = currentScreenSectorChunkPosMax.CurrentPosition - previousScreenSectorChunkPosMax.CurrentPosition;
            }

            Vector2Int screenFilamentChunkPosMinOffset = Vector2Int.zero;
            Vector2Int screenFilamentChunkPosMaxOffset = Vector2Int.zero;
            if (currentScreenFilamentChunkPosMin != previousScreenFilamentChunkPosMin)
            {
                screenFilamentChunkPosMinOffset = currentScreenFilamentChunkPosMin.CurrentPosition - previousScreenFilamentChunkPosMin.CurrentPosition;
            }
            if (currentScreenFilamentChunkPosMax != previousScreenFilamentChunkPosMax)
            {
                screenFilamentChunkPosMaxOffset = currentScreenFilamentChunkPosMax.CurrentPosition - previousScreenFilamentChunkPosMax.CurrentPosition;
            }

            Vector2Int screenRegionPosMinOffset = Vector2Int.zero;
            Vector2Int screenRegionPosMaxOffset = Vector2Int.zero;
            if (currentScreenRegionPosMin != previousScreenRegionPosMin)
            {
                screenRegionPosMinOffset = currentScreenRegionPosMin.CurrentPosition - previousScreenRegionPosMin.CurrentPosition;
            }
            if (currentScreenRegionPosMax != previousScreenRegionPosMax)
            {
                screenRegionPosMaxOffset = currentScreenRegionPosMax.CurrentPosition - previousScreenRegionPosMax.CurrentPosition;
            }

            Vector2Int screenSectorPosMinOffset = Vector2Int.zero;
            Vector2Int screenSectorPosMaxOffset = Vector2Int.zero;
            if (currentScreenSectorPosMin != previousScreenSectorPosMin)
            {
                screenSectorPosMinOffset = currentScreenSectorPosMin.CurrentPosition - previousScreenSectorPosMin.CurrentPosition;
            }
            if (currentScreenSectorPosMax != previousScreenSectorPosMax)
            {
                screenSectorPosMaxOffset = currentScreenSectorPosMax.CurrentPosition - previousScreenSectorPosMax.CurrentPosition;
            }

            Vector2Int screenFilamentPosMinOffset = Vector2Int.zero;
            Vector2Int screenFilamentPosMaxOffset = Vector2Int.zero;
            if (currentScreenFilamentPosMin != previousScreenFilamentPosMin)
            {
                screenFilamentPosMinOffset = currentScreenFilamentPosMin.CurrentPosition - previousScreenFilamentPosMin.CurrentPosition;
            }
            if (currentScreenFilamentPosMax != previousScreenFilamentPosMax)
            {
                screenFilamentPosMaxOffset = currentScreenFilamentPosMax.CurrentPosition - previousScreenFilamentPosMax.CurrentPosition;
            }
            #endregion

            #region Calculate Previously & Newly Proximal Screen Positions
            if (screenRegionChunkPosMinOffset != Vector2Int.zero || screenRegionChunkPosMaxOffset != Vector2Int.zero)
            {
                #region Previously Proximal Region Chunks
                previouslyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();
                Parallel.For(previousScreenRegionChunkPosMin.CurrentPosition.x, previousScreenRegionChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(previousScreenRegionChunkPosMin.CurrentPosition.y, previousScreenRegionChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < currentScreenRegionChunkPosMin.CurrentPosition.x || x > currentScreenRegionChunkPosMax.CurrentPosition.x || y < currentScreenRegionChunkPosMin.CurrentPosition.y || y > currentScreenRegionChunkPosMax.CurrentPosition.y)
                        {
                            previouslyProximalRegionChunkPositions.Add(new Universe.Region.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion

                #region Newly Proximal Region Chunks
                newlyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();
                Parallel.For(currentScreenRegionChunkPosMin.CurrentPosition.x, currentScreenRegionChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenRegionChunkPosMin.CurrentPosition.y, currentScreenRegionChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenRegionChunkPosMin.CurrentPosition.x || x > previousScreenRegionChunkPosMax.CurrentPosition.x || y < previousScreenRegionChunkPosMin.CurrentPosition.y || y > previousScreenRegionChunkPosMax.CurrentPosition.y)
                        {
                            newlyProximalRegionChunkPositions.Add(new Universe.Region.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion
            }

            if (screenSectorChunkPosMinOffset != Vector2Int.zero || screenSectorChunkPosMaxOffset != Vector2Int.zero)
            {
                #region Previously Proximal Sector Chunks
                previouslyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();
                Parallel.For(previousScreenSectorChunkPosMin.CurrentPosition.x, previousScreenSectorChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(previousScreenSectorChunkPosMin.CurrentPosition.y, previousScreenSectorChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < currentScreenSectorChunkPosMin.CurrentPosition.x || x > currentScreenSectorChunkPosMax.CurrentPosition.x || y < currentScreenSectorChunkPosMin.CurrentPosition.y || y > currentScreenSectorChunkPosMax.CurrentPosition.y)
                        {
                            previouslyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion

                #region Newly Proximal Sector Chunks
                newlyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();
                Parallel.For(currentScreenSectorChunkPosMin.CurrentPosition.x, currentScreenSectorChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenSectorChunkPosMin.CurrentPosition.y, currentScreenSectorChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenSectorChunkPosMin.CurrentPosition.x || x > previousScreenSectorChunkPosMax.CurrentPosition.x || y < previousScreenSectorChunkPosMin.CurrentPosition.y || y > previousScreenSectorChunkPosMax.CurrentPosition.y)
                        {
                            newlyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion
            }

            if (screenFilamentChunkPosMinOffset != Vector2Int.zero || screenFilamentChunkPosMaxOffset != Vector2Int.zero)
            {
                #region Previously Proximal Filament Chunks
                previouslyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();
                Parallel.For(previousScreenFilamentChunkPosMin.CurrentPosition.x, previousScreenFilamentChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(previousScreenFilamentChunkPosMin.CurrentPosition.y, previousScreenFilamentChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < currentScreenFilamentChunkPosMin.CurrentPosition.x || x > currentScreenFilamentChunkPosMax.CurrentPosition.x || y < currentScreenFilamentChunkPosMin.CurrentPosition.y || y > currentScreenFilamentChunkPosMax.CurrentPosition.y)
                        {
                            previouslyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion

                #region Newly Proximal Filament Chunks
                newlyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();
                Parallel.For(currentScreenFilamentChunkPosMin.CurrentPosition.x, currentScreenFilamentChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenFilamentChunkPosMin.CurrentPosition.y, currentScreenFilamentChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenFilamentChunkPosMin.CurrentPosition.x || x > previousScreenFilamentChunkPosMax.CurrentPosition.x || y < previousScreenFilamentChunkPosMin.CurrentPosition.y || y > previousScreenFilamentChunkPosMax.CurrentPosition.y)
                        {
                            newlyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion
            }

            if (screenRegionPosMinOffset != Vector2Int.zero || screenRegionPosMaxOffset != Vector2Int.zero)
            {
                #region Previously Proximal Regions
                previouslyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();
                Parallel.For(previousScreenRegionPosMin.CurrentPosition.x, previousScreenRegionPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(previousScreenRegionPosMin.CurrentPosition.y, previousScreenRegionPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < currentScreenRegionPosMin.CurrentPosition.x || x > currentScreenRegionPosMax.CurrentPosition.x || y < currentScreenRegionPosMin.CurrentPosition.y || y > currentScreenRegionPosMax.CurrentPosition.y)
                        {
                            previouslyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion

                #region Newly Proximal Regions
                newlyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();
                Parallel.For(currentScreenRegionPosMin.CurrentPosition.x, currentScreenRegionPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenRegionPosMin.CurrentPosition.y, currentScreenRegionPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenRegionPosMin.CurrentPosition.x || x > previousScreenRegionPosMax.CurrentPosition.x || y < previousScreenRegionPosMin.CurrentPosition.y || y > previousScreenRegionPosMax.CurrentPosition.y)
                        {
                            newlyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion
            }

            if (screenSectorPosMinOffset != Vector2Int.zero || screenSectorPosMaxOffset != Vector2Int.zero)
            {
                #region Previously Proximal Sectors
                previouslyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();
                Parallel.For(previousScreenSectorPosMin.CurrentPosition.x, previousScreenSectorPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(previousScreenSectorPosMin.CurrentPosition.y, previousScreenSectorPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < currentScreenSectorPosMin.CurrentPosition.x || x > currentScreenSectorPosMax.CurrentPosition.x || y < currentScreenSectorPosMin.CurrentPosition.y || y > currentScreenSectorPosMax.CurrentPosition.y)
                        {
                            previouslyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion

                #region Newly Proximal Sectors
                newlyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();
                Parallel.For(currentScreenSectorPosMin.CurrentPosition.x, currentScreenSectorPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenSectorPosMin.CurrentPosition.y, currentScreenSectorPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenSectorPosMin.CurrentPosition.x || x > previousScreenSectorPosMax.CurrentPosition.x || y < previousScreenSectorPosMin.CurrentPosition.y || y > previousScreenSectorPosMax.CurrentPosition.y)
                        {
                            newlyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion
            }

            if (screenFilamentPosMinOffset != Vector2Int.zero || screenFilamentPosMaxOffset != Vector2Int.zero)
            {
                #region Previously Proximal Filaments
                previouslyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();
                Parallel.For(previousScreenFilamentPosMin.CurrentPosition.x, previousScreenFilamentPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(previousScreenFilamentPosMin.CurrentPosition.y, previousScreenFilamentPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < currentScreenFilamentPosMin.CurrentPosition.x || x > currentScreenFilamentPosMax.CurrentPosition.x || y < currentScreenFilamentPosMin.CurrentPosition.y || y > currentScreenFilamentPosMax.CurrentPosition.y)
                        {
                            previouslyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion

                #region Newly Proximal Filaments
                newlyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();
                Parallel.For(currentScreenFilamentPosMin.CurrentPosition.x, currentScreenFilamentPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenFilamentPosMin.CurrentPosition.y, currentScreenFilamentPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenFilamentPosMin.CurrentPosition.x || x > previousScreenFilamentPosMax.CurrentPosition.x || y < previousScreenFilamentPosMin.CurrentPosition.y || y > previousScreenFilamentPosMax.CurrentPosition.y)
                        {
                            newlyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                #endregion
            }
            #endregion

            #region Update Previous Screen Positions
            previousScreenRegionChunkPosMin = currentScreenRegionChunkPosMin;
            previousScreenRegionChunkPosMax = currentScreenRegionChunkPosMax;

            previousScreenSectorChunkPosMin = currentScreenSectorChunkPosMin;
            previousScreenSectorChunkPosMax = currentScreenSectorChunkPosMax;

            previousScreenFilamentChunkPosMin = currentScreenFilamentChunkPosMin;
            previousScreenFilamentChunkPosMax = currentScreenFilamentChunkPosMax;

            previousScreenRegionPosMin = currentScreenRegionPosMin;
            previousScreenRegionPosMax = currentScreenRegionPosMax;

            previousScreenSectorPosMin = currentScreenSectorPosMin;
            previousScreenSectorPosMax = currentScreenSectorPosMax;

            previousScreenFilamentPosMin = currentScreenFilamentPosMin;
            previousScreenFilamentPosMax = currentScreenFilamentPosMax;
            #endregion
        }
        
        private void LoadNewlyProximalPositions()
        {
            Benchmark.Start("LoadPositions");

            Benchmark.Start("LoadFilament");
            foreach (var proximalFilamentPosition in newlyProximalFilamentPositions)
            {
                if (!currentUniverse.IsFilamentGenerated(proximalFilamentPosition))
                {
                    currentUniverse.RequestGenerateFilament(proximalFilamentPosition);
                }
                else if (!currentUniverse.IsFilamentLoaded(proximalFilamentPosition))
                {
                    currentUniverse.LoadFilament(proximalFilamentPosition);
                }
            }
            Benchmark.Stop("LoadFilament");

            Benchmark.Start("LoadFilamentChunk");
            foreach (var proximalFilamentChunkPosition in newlyProximalFilamentChunkPositions)
            {
                if (!currentUniverse.IsFilamentChunkGenerated(proximalFilamentChunkPosition))
                {
                    currentUniverse.RequestGenerateFilamentChunk(proximalFilamentChunkPosition);
                }
                else if (!currentUniverse.IsFilamentChunkLoaded(proximalFilamentChunkPosition))
                {
                    currentUniverse.LoadFilamentChunk(proximalFilamentChunkPosition);
                }
            }
            Benchmark.Stop("LoadFilamentChunk");

            Benchmark.Start("LoadSector");
            foreach (var proximalSectorPosition in newlyProximalSectorPositions)
            {
                if (!currentUniverse.IsSectorGenerated(proximalSectorPosition))
                {
                    currentUniverse.RequestGenerateSector(proximalSectorPosition);
                }
                else if (!currentUniverse.IsSectorLoaded(proximalSectorPosition))
                {
                    currentUniverse.LoadSector(proximalSectorPosition);
                }
            }
            Benchmark.Stop("LoadSector");

            Benchmark.Start("LoadSectorChunk");
            foreach (var proximalSectorChunkPosition in newlyProximalSectorChunkPositions)
            {
                if (!currentUniverse.IsSectorChunkGenerated(proximalSectorChunkPosition))
                {
                    currentUniverse.RequestGenerateSectorChunk(proximalSectorChunkPosition);
                }
                else if (!currentUniverse.IsSectorChunkLoaded(proximalSectorChunkPosition))
                {
                    currentUniverse.LoadSectorChunk(proximalSectorChunkPosition);
                }
            }
            Benchmark.Stop("LoadSectorChunk");

            Benchmark.Start("LoadRegion");
            foreach (var proximalRegionPosition in newlyProximalRegionPositions)
            {
                if (!currentUniverse.IsRegionGenerated(proximalRegionPosition))
                {
                    currentUniverse.RequestGenerateRegion(proximalRegionPosition);
                }
                else if (!currentUniverse.IsRegionLoaded(proximalRegionPosition))
                {
                    currentUniverse.LoadRegion(proximalRegionPosition);
                }
            }
            Benchmark.Stop("LoadRegion");

            Benchmark.Start("LoadRegionChunk");
            foreach (var proximalRegionChunkPosition in newlyProximalRegionChunkPositions)
            {
                if (!currentUniverse.IsRegionChunkGenerated(proximalRegionChunkPosition))
                {
                    currentUniverse.RequestGenerateRegionChunk(proximalRegionChunkPosition);
                }
                else if (!currentUniverse.IsRegionChunkLoaded(proximalRegionChunkPosition))
                {
                    currentUniverse.LoadRegionChunk(proximalRegionChunkPosition);
                }
            }
            Benchmark.Stop("LoadRegionChunk");

            Benchmark.Stop("LoadPositions");
        }

        private void UnloadPreviouslyProximalPositions()
        {
            Benchmark.Start("UnloadPositions");

            // Region Chunk
            foreach (Universe.Region.Chunk.Position previouslyProximalRegionChunkPosition in previouslyProximalRegionChunkPositions)
            {
                if (currentUniverse.IsRegionChunkLoaded(previouslyProximalRegionChunkPosition))
                {
                    currentUniverse.UnloadRegionChunk(previouslyProximalRegionChunkPosition);
                }
            }

            // Region
            foreach (Universe.Region.Position previouslyProximalRegionPosition in previouslyProximalRegionPositions)
            {
                if (currentUniverse.IsRegionLoaded(previouslyProximalRegionPosition))
                {
                    currentUniverse.UnloadRegion(previouslyProximalRegionPosition);
                }
            }

            // Sector Chunk
            foreach (Universe.Sector.Chunk.Position previouslyProximalSectorChunkPosition in previouslyProximalSectorChunkPositions)
            {
                if (currentUniverse.IsSectorChunkLoaded(previouslyProximalSectorChunkPosition))
                {
                    currentUniverse.UnloadSectorChunk(previouslyProximalSectorChunkPosition);
                }
            }

            // Sector
            foreach (Universe.Sector.Position previouslyProximalSectorPosition in previouslyProximalSectorPositions)
            {
                if (currentUniverse.IsSectorLoaded(previouslyProximalSectorPosition))
                {
                    currentUniverse.UnloadSector(previouslyProximalSectorPosition);
                }
            }

            // Filament Chunk
            foreach (Universe.Filament.Chunk.Position previouslyProximalFilamentChunkPosition in previouslyProximalFilamentChunkPositions)
            {
                if (currentUniverse.IsFilamentChunkLoaded(previouslyProximalFilamentChunkPosition))
                {
                    currentUniverse.UnloadFilamentChunk(previouslyProximalFilamentChunkPosition);
                }
            }

            // Filament
            foreach (Universe.Filament.Position previouslyProximalFilamentPosition in previouslyProximalFilamentPositions)
            {
                if (currentUniverse.IsFilamentLoaded(previouslyProximalFilamentPosition))
                {
                    currentUniverse.UnloadFilament(previouslyProximalFilamentPosition);
                }
            }

            Benchmark.Stop("UnloadPositions");
        }

        private void DrawLoadedPositionGizmos()
        {
            int regionSize = currentUniverse.RegionGenerationSettings.Size;
            int regionChunkSize = currentUniverse.RegionGenerationSettings.ChunkSize;
            int sectorSize = currentUniverse.SectorGenerationSettings.Size;
            int sectorChunkSize = currentUniverse.SectorGenerationSettings.ChunkSize;
            int filamentSize = currentUniverse.FilamentGenerationSettings.Size;
            int filamentChunkSize = currentUniverse.FilamentGenerationSettings.ChunkSize;

            // Region Chunk
            foreach (Universe.Region.Chunk.Position loadedRegionChunkPositions in currentUniverse.LoadedRegionChunks.Keys)
            {
                Gizmos.color = Color.green;
                Gizmos.DrawWireCube(loadedRegionChunkPositions.WorldPosition, new Vector2(regionChunkSize, regionChunkSize));
            }

            // Region
            foreach (Universe.Region.Position loadedRegionPositions in currentUniverse.LoadedRegions.Keys)
            {
                Gizmos.color = Color.red;
                Gizmos.DrawWireCube(loadedRegionPositions.WorldPosition, new Vector2(regionSize, regionSize));
            }

            // Sector Chunk
            foreach (Universe.Sector.Chunk.Position loadedSectorChunkPositions in currentUniverse.LoadedSectorChunks.Keys)
            {
                Gizmos.color = Color.yellow;
                Gizmos.DrawWireCube(loadedSectorChunkPositions.WorldPosition, new Vector2(sectorChunkSize, sectorChunkSize));
            }

            // Sector
            foreach (Universe.Sector.Position loadedSectorPositions in currentUniverse.LoadedSectors.Keys)
            {
                Gizmos.color = Color.blue;
                Gizmos.DrawWireCube(loadedSectorPositions.WorldPosition, new Vector2(sectorSize, sectorSize));
            }

            // Filament Chunk
            foreach (Universe.Filament.Chunk.Position loadedFilamentChunkPositions in currentUniverse.LoadedFilamentChunks.Keys)
            {
                Gizmos.color = new Color(1.0f, 0.0f, 1.0f);
                Gizmos.DrawWireCube(loadedFilamentChunkPositions.WorldPosition, new Vector2(filamentChunkSize, filamentChunkSize));
            }

            // Filament
            foreach (Universe.Filament.Position loadedFilamentPositions in currentUniverse.LoadedFilaments.Keys)
            {
                Gizmos.color = new Color(1.0f, 0.5f, 0.0f);
                Gizmos.DrawWireCube(loadedFilamentPositions.WorldPosition, new Vector2(filamentSize, filamentSize));
            }
        }

        private void PrintBenchmarks()
        {
            Debug.Log(
                $"ELEMENT LOAD:" +
                $"\t\t\t\tRegion: \t\t{Benchmark.AverageDuration("LoadRegion").Milliseconds}({Benchmark.MaxDuration("LoadRegion").Milliseconds})ms" +
                $"\t\t\t\tChunk: \t{Benchmark.AverageDuration("LoadRegionChunk").Milliseconds}({Benchmark.MaxDuration("LoadRegionChunk").Milliseconds})ms");
            Debug.Log(
                $"ELEMENT LOAD:" +
                $"\t\t\t\tSector: \t\t{Benchmark.AverageDuration("LoadSector").Milliseconds}({Benchmark.MaxDuration("LoadSector").Milliseconds})ms" +
                $"\t\t\t\tChunk: \t{Benchmark.AverageDuration("LoadSectorChunk").Milliseconds}({Benchmark.MaxDuration("LoadSectorChunk").Milliseconds})ms");
            Debug.Log(
                $"ELEMENT LOAD:" +
                $"\t\t\t\tFilament: \t{Benchmark.AverageDuration("LoadFilament").Milliseconds}({Benchmark.MaxDuration("LoadFilament").Milliseconds})ms" +
                $"\t\t\t\tChunk: \t{Benchmark.AverageDuration("LoadFilamentChunk").Milliseconds}({Benchmark.MaxDuration("LoadFilamentChunk").Milliseconds})ms");
            Debug.Log(
                $"MISCELLANEOUS:" +
                //$"\t\t\t\tUpdate Position: \t{Benchmark.AverageDuration("UpdatePosition").Milliseconds}({Benchmark.MaxDuration("UpdatePosition").Milliseconds})ms" +
                $"\t\t Update Positions: \t{Benchmark.AverageDuration("UpdatePositions").Milliseconds}({Benchmark.MaxDuration("UpdatePositions").Milliseconds})ms" +
                $"\t\t Load Positions: \t{Benchmark.AverageDuration("LoadPositions").Milliseconds}({Benchmark.MaxDuration("LoadPositions").Milliseconds})ms" +
                $"\t\t Unload Positions: \t{Benchmark.AverageDuration("UnloadPositions").Milliseconds}({Benchmark.MaxDuration("UnloadPositions").Milliseconds})ms");
        }
    }
}
