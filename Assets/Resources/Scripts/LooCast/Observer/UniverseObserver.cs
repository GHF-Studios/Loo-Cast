using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.Concurrent;
using System.Threading.Tasks;
using System.Linq;
using UnityEngine;
using UnityEngine.Profiling;

namespace LooCast.Observer
{
    using Core;
    using Game;
    using LooCast.Diagnostic;
    using Universe;

    public class UniverseObserver : ExtendedMonoBehaviour
    {
        #region Fields
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

        private ConcurrentBag<Universe.Region.Chunk.Position> currentlyProximalRegionChunkPositions;
        private ConcurrentBag<Universe.Sector.Chunk.Position> currentlyProximalSectorChunkPositions;
        private ConcurrentBag<Universe.Filament.Chunk.Position> currentlyProximalFilamentChunkPositions;
        private ConcurrentBag<Universe.Region.Position> currentlyProximalRegionPositions;
        private ConcurrentBag<Universe.Sector.Position> currentlyProximalSectorPositions;
        private ConcurrentBag<Universe.Filament.Position> currentlyProximalFilamentPositions;

        private ConcurrentBag<Universe.Region.Chunk.Position> newlyProximalRegionChunkPositions;
        private ConcurrentBag<Universe.Sector.Chunk.Position> newlyProximalSectorChunkPositions;
        private ConcurrentBag<Universe.Filament.Chunk.Position> newlyProximalFilamentChunkPositions;
        private ConcurrentBag<Universe.Region.Position> newlyProximalRegionPositions;
        private ConcurrentBag<Universe.Sector.Position> newlyProximalSectorPositions;
        private ConcurrentBag<Universe.Filament.Position> newlyProximalFilamentPositions;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            currentUniverse = GameManager.Instance.CurrentGame.CurrentUniverse;
            regionChunkLoadRadius = 2;
            // TODO: Implement Sector and Filament Chunk Load Radius

            InitializeScreenPositions();
            InitializeProximalPositions();
            StartCoroutine(LoadNewlyProximalPositionsCoroutine());
        }

        private void Update()
        {
            UpdateScreenPositions();

            UpdateProximalPositions();

            UnloadPreviouslyProximalPositions();

            // TODO: Fix / Maybe remove CancelInvalidatedProximalPositionLoadRequests
            // CancelInvalidatedProximalPositionLoadRequests();
        }

        private void OnDrawGizmos()
        {
            DrawLoadedPositionGizmos();
        }
        #endregion

        #region Methods
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

            currentlyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();
            currentlyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();
            currentlyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();
            currentlyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();
            currentlyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();
            currentlyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();

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
            newlyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>(newlyProximalRegionChunkPositions.OrderBy((newlyProximalRegionChunkPosition) =>
            {
                return -(new Universe.Region.Chunk.Position(transform.position).WorldPosition - newlyProximalRegionChunkPosition.WorldPosition).sqrMagnitude;
            }));

            Parallel.For(previousScreenSectorChunkPosMin.CurrentPosition.x, previousScreenSectorChunkPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenSectorChunkPosMin.CurrentPosition.y, previousScreenSectorChunkPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(x, y)));
                });
            });
            newlyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>(newlyProximalSectorChunkPositions.OrderBy((newlyProximalSectorChunkPosition) =>
            {
                return -(new Universe.Sector.Chunk.Position(transform.position).WorldPosition - newlyProximalSectorChunkPosition.WorldPosition).sqrMagnitude;
            }));

            Parallel.For(previousScreenFilamentChunkPosMin.CurrentPosition.x, previousScreenFilamentChunkPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenFilamentChunkPosMin.CurrentPosition.y, previousScreenFilamentChunkPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(x, y)));
                });
            });
            newlyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>(newlyProximalFilamentChunkPositions.OrderBy((newlyProximalFilamentChunkPosition) =>
            {
                return -(new Universe.Filament.Chunk.Position(transform.position).WorldPosition - newlyProximalFilamentChunkPosition.WorldPosition).sqrMagnitude;
            }));

            Parallel.For(previousScreenRegionPosMin.CurrentPosition.x, previousScreenRegionPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenRegionPosMin.CurrentPosition.y, previousScreenRegionPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(x, y)));
                });
            });
            newlyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>(newlyProximalRegionPositions.OrderBy((newlyProximalRegionPosition) =>
            {
                return -(new Universe.Region.Position(transform.position).WorldPosition - newlyProximalRegionPosition.WorldPosition).sqrMagnitude;
            }));

            Parallel.For(previousScreenSectorPosMin.CurrentPosition.x, previousScreenSectorPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenSectorPosMin.CurrentPosition.y, previousScreenSectorPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(x, y)));
                });
            });
            newlyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>(newlyProximalSectorPositions.OrderBy((newlyProximalSectorPosition) =>
            {
                return -(new Universe.Sector.Position(transform.position).WorldPosition - newlyProximalSectorPosition.WorldPosition).sqrMagnitude;
            }));

            Parallel.For(previousScreenFilamentPosMin.CurrentPosition.x, previousScreenFilamentPosMax.CurrentPosition.x + 1, (x) =>
            {
                Parallel.For(previousScreenFilamentPosMin.CurrentPosition.y, previousScreenFilamentPosMax.CurrentPosition.y + 1, (y) =>
                {
                    newlyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(x, y)));
                });
            });
            newlyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>(newlyProximalFilamentPositions.OrderBy((newlyProximalFilamentPosition) =>
            {
                return -(new Universe.Filament.Position(transform.position).WorldPosition - newlyProximalFilamentPosition.WorldPosition).sqrMagnitude;
            }));
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

            #region Calculate Previously, Currently, & Newly Proximal Screen Positions
            if (screenRegionChunkPosMinOffset != Vector2Int.zero || screenRegionChunkPosMaxOffset != Vector2Int.zero)
            {
                #region Newly & Currently Proximal Region Chunks
                newlyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();
                Parallel.For(currentScreenRegionChunkPosMin.CurrentPosition.x, currentScreenRegionChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenRegionChunkPosMin.CurrentPosition.y, currentScreenRegionChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenRegionChunkPosMin.CurrentPosition.x || x > previousScreenRegionChunkPosMax.CurrentPosition.x || y < previousScreenRegionChunkPosMin.CurrentPosition.y || y > previousScreenRegionChunkPosMax.CurrentPosition.y)
                        {
                            newlyProximalRegionChunkPositions.Add(new Universe.Region.Chunk.Position(new Vector2Int(x, y)));
                        }
                        if (x < previousScreenRegionChunkPosMin.CurrentPosition.x && x > previousScreenRegionChunkPosMax.CurrentPosition.x && y < previousScreenRegionChunkPosMin.CurrentPosition.y && y > previousScreenRegionChunkPosMax.CurrentPosition.y)
                        {
                            currentlyProximalRegionChunkPositions.Add(new Universe.Region.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                newlyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>(newlyProximalRegionChunkPositions.OrderBy((newlyProximalRegionChunkPosition) =>
                {
                    return -(new Universe.Region.Chunk.Position(transform.position).WorldPosition - newlyProximalRegionChunkPosition.WorldPosition).sqrMagnitude;
                }));
                #endregion
                
                #region Previously Proximal Region Chunks
                previouslyProximalRegionChunkPositions = new ConcurrentBag<Universe.Region.Chunk.Position>();

                Parallel.ForEach(currentUniverse.LoadedRegionChunks.Keys.Except(currentlyProximalRegionChunkPositions), (loadedRegionChunkPosition) =>
                {
                    if (loadedRegionChunkPosition.CurrentPosition.x < currentScreenRegionChunkPosMin.CurrentPosition.x || loadedRegionChunkPosition.CurrentPosition.x > currentScreenRegionChunkPosMax.CurrentPosition.x || loadedRegionChunkPosition.CurrentPosition.y < currentScreenRegionChunkPosMin.CurrentPosition.y || loadedRegionChunkPosition.CurrentPosition.y > currentScreenRegionChunkPosMax.CurrentPosition.y)
                    {
                        previouslyProximalRegionChunkPositions.Add(new Universe.Region.Chunk.Position(new Vector2Int(loadedRegionChunkPosition.CurrentPosition.x, loadedRegionChunkPosition.CurrentPosition.y)));
                    }
                });
                #endregion
            }

            if (screenSectorChunkPosMinOffset != Vector2Int.zero || screenSectorChunkPosMaxOffset != Vector2Int.zero)
            {
                #region Newly & Currently Proximal Sector Chunks
                newlyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();
                Parallel.For(currentScreenSectorChunkPosMin.CurrentPosition.x, currentScreenSectorChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenSectorChunkPosMin.CurrentPosition.y, currentScreenSectorChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenSectorChunkPosMin.CurrentPosition.x || x > previousScreenSectorChunkPosMax.CurrentPosition.x || y < previousScreenSectorChunkPosMin.CurrentPosition.y || y > previousScreenSectorChunkPosMax.CurrentPosition.y)
                        {
                            newlyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(x, y)));
                        }
                        if (x < previousScreenSectorChunkPosMin.CurrentPosition.x && x > previousScreenSectorChunkPosMax.CurrentPosition.x && y < previousScreenSectorChunkPosMin.CurrentPosition.y && y > previousScreenSectorChunkPosMax.CurrentPosition.y)
                        {
                            currentlyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                newlyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>(newlyProximalSectorChunkPositions.OrderBy((newlyProximalSectorChunkPosition) =>
                {
                    return -(new Universe.Sector.Chunk.Position(transform.position).WorldPosition - newlyProximalSectorChunkPosition.WorldPosition).sqrMagnitude;
                }));
                #endregion
                
                #region Previously Proximal Sector Chunks
                previouslyProximalSectorChunkPositions = new ConcurrentBag<Universe.Sector.Chunk.Position>();

                Parallel.ForEach(currentUniverse.LoadedSectorChunks.Keys.Except(currentlyProximalSectorChunkPositions), (loadedSectorChunkPosition) =>
                {
                    if (loadedSectorChunkPosition.CurrentPosition.x < currentScreenSectorChunkPosMin.CurrentPosition.x || loadedSectorChunkPosition.CurrentPosition.x > currentScreenSectorChunkPosMax.CurrentPosition.x || loadedSectorChunkPosition.CurrentPosition.y < currentScreenSectorChunkPosMin.CurrentPosition.y || loadedSectorChunkPosition.CurrentPosition.y > currentScreenSectorChunkPosMax.CurrentPosition.y)
                    {
                        previouslyProximalSectorChunkPositions.Add(new Universe.Sector.Chunk.Position(new Vector2Int(loadedSectorChunkPosition.CurrentPosition.x, loadedSectorChunkPosition.CurrentPosition.y)));
                    }
                });
                #endregion
            }

            if (screenFilamentChunkPosMinOffset != Vector2Int.zero || screenFilamentChunkPosMaxOffset != Vector2Int.zero)
            {
                #region Newly & Currently Proximal Filament Chunks
                newlyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();
                Parallel.For(currentScreenFilamentChunkPosMin.CurrentPosition.x, currentScreenFilamentChunkPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenFilamentChunkPosMin.CurrentPosition.y, currentScreenFilamentChunkPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenFilamentChunkPosMin.CurrentPosition.x || x > previousScreenFilamentChunkPosMax.CurrentPosition.x || y < previousScreenFilamentChunkPosMin.CurrentPosition.y || y > previousScreenFilamentChunkPosMax.CurrentPosition.y)
                        {
                            newlyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(x, y)));
                        }
                        if (x < previousScreenFilamentChunkPosMin.CurrentPosition.x && x > previousScreenFilamentChunkPosMax.CurrentPosition.x && y < previousScreenFilamentChunkPosMin.CurrentPosition.y && y > previousScreenFilamentChunkPosMax.CurrentPosition.y)
                        {
                            currentlyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                newlyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>(newlyProximalFilamentChunkPositions.OrderBy((newlyProximalFilamentChunkPosition) =>
                {
                    return -(new Universe.Filament.Chunk.Position(transform.position).WorldPosition - newlyProximalFilamentChunkPosition.WorldPosition).sqrMagnitude;
                }));
                #endregion
                
                #region Previously Proximal Filament Chunks
                previouslyProximalFilamentChunkPositions = new ConcurrentBag<Universe.Filament.Chunk.Position>();

                Parallel.ForEach(currentUniverse.LoadedFilamentChunks.Keys.Except(currentlyProximalFilamentChunkPositions), (loadedFilamentChunkPosition) =>
                {
                    if (loadedFilamentChunkPosition.CurrentPosition.x < currentScreenFilamentChunkPosMin.CurrentPosition.x || loadedFilamentChunkPosition.CurrentPosition.x > currentScreenFilamentChunkPosMax.CurrentPosition.x || loadedFilamentChunkPosition.CurrentPosition.y < currentScreenFilamentChunkPosMin.CurrentPosition.y || loadedFilamentChunkPosition.CurrentPosition.y > currentScreenFilamentChunkPosMax.CurrentPosition.y)
                    {
                        previouslyProximalFilamentChunkPositions.Add(new Universe.Filament.Chunk.Position(new Vector2Int(loadedFilamentChunkPosition.CurrentPosition.x, loadedFilamentChunkPosition.CurrentPosition.y)));
                    }
                });
                #endregion
            }

            if (screenRegionPosMinOffset != Vector2Int.zero || screenRegionPosMaxOffset != Vector2Int.zero)
            {
                #region Newly & Currently Proximal Regions
                newlyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();
                Parallel.For(currentScreenRegionPosMin.CurrentPosition.x, currentScreenRegionPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenRegionPosMin.CurrentPosition.y, currentScreenRegionPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenRegionPosMin.CurrentPosition.x || x > previousScreenRegionPosMax.CurrentPosition.x || y < previousScreenRegionPosMin.CurrentPosition.y || y > previousScreenRegionPosMax.CurrentPosition.y)
                        {
                            newlyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(x, y)));
                        }
                        if (x < previousScreenRegionPosMin.CurrentPosition.x && x > previousScreenRegionPosMax.CurrentPosition.x && y < previousScreenRegionPosMin.CurrentPosition.y && y > previousScreenRegionPosMax.CurrentPosition.y)
                        {
                            currentlyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                newlyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>(newlyProximalRegionPositions.OrderBy((newlyProximalRegionPosition) =>
                {
                    return -(new Universe.Region.Position(transform.position).WorldPosition - newlyProximalRegionPosition.WorldPosition).sqrMagnitude;
                }));
                #endregion
                
                #region Previously Proximal Regions
                previouslyProximalRegionPositions = new ConcurrentBag<Universe.Region.Position>();

                Parallel.ForEach(currentUniverse.LoadedRegions.Keys.Except(currentlyProximalRegionPositions), (loadedRegionPosition) =>
                {
                    if (loadedRegionPosition.CurrentPosition.x < currentScreenRegionPosMin.CurrentPosition.x || loadedRegionPosition.CurrentPosition.x > currentScreenRegionPosMax.CurrentPosition.x || loadedRegionPosition.CurrentPosition.y < currentScreenRegionPosMin.CurrentPosition.y || loadedRegionPosition.CurrentPosition.y > currentScreenRegionPosMax.CurrentPosition.y)
                    {
                        previouslyProximalRegionPositions.Add(new Universe.Region.Position(new Vector2Int(loadedRegionPosition.CurrentPosition.x, loadedRegionPosition.CurrentPosition.y)));
                    }
                });
                #endregion

            }

            if (screenSectorPosMinOffset != Vector2Int.zero || screenSectorPosMaxOffset != Vector2Int.zero)
            {
                #region Newly & Currently Proximal Sectors
                newlyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();
                Parallel.For(currentScreenSectorPosMin.CurrentPosition.x, currentScreenSectorPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenSectorPosMin.CurrentPosition.y, currentScreenSectorPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenSectorPosMin.CurrentPosition.x || x > previousScreenSectorPosMax.CurrentPosition.x || y < previousScreenSectorPosMin.CurrentPosition.y || y > previousScreenSectorPosMax.CurrentPosition.y)
                        {
                            newlyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(x, y)));
                        }
                        if (x < previousScreenSectorPosMin.CurrentPosition.x && x > previousScreenSectorPosMax.CurrentPosition.x && y < previousScreenSectorPosMin.CurrentPosition.y && y > previousScreenSectorPosMax.CurrentPosition.y)
                        {
                            currentlyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                newlyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>(newlyProximalSectorPositions.OrderBy((newlyProximalSectorPosition) =>
                {
                    return -(new Universe.Sector.Position(transform.position).WorldPosition - newlyProximalSectorPosition.WorldPosition).sqrMagnitude;
                }));
                #endregion
                
                #region Previously Proximal Sectors
                previouslyProximalSectorPositions = new ConcurrentBag<Universe.Sector.Position>();

                Parallel.ForEach(currentUniverse.LoadedSectors.Keys.Except(currentlyProximalSectorPositions), (loadedSectorPosition) =>
                {
                    if (loadedSectorPosition.CurrentPosition.x < currentScreenSectorPosMin.CurrentPosition.x || loadedSectorPosition.CurrentPosition.x > currentScreenSectorPosMax.CurrentPosition.x || loadedSectorPosition.CurrentPosition.y < currentScreenSectorPosMin.CurrentPosition.y || loadedSectorPosition.CurrentPosition.y > currentScreenSectorPosMax.CurrentPosition.y)
                    {
                        previouslyProximalSectorPositions.Add(new Universe.Sector.Position(new Vector2Int(loadedSectorPosition.CurrentPosition.x, loadedSectorPosition.CurrentPosition.y)));
                    }
                });
                #endregion

            }

            if (screenFilamentPosMinOffset != Vector2Int.zero || screenFilamentPosMaxOffset != Vector2Int.zero)
            {
                #region Newly & Currently Proximal Filaments
                newlyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();
                Parallel.For(currentScreenFilamentPosMin.CurrentPosition.x, currentScreenFilamentPosMax.CurrentPosition.x + 1, (x) =>
                {
                    Parallel.For(currentScreenFilamentPosMin.CurrentPosition.y, currentScreenFilamentPosMax.CurrentPosition.y + 1, (y) =>
                    {
                        if (x < previousScreenFilamentPosMin.CurrentPosition.x || x > previousScreenFilamentPosMax.CurrentPosition.x || y < previousScreenFilamentPosMin.CurrentPosition.y || y > previousScreenFilamentPosMax.CurrentPosition.y)
                        {
                            newlyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(x, y)));
                        }
                        if (x < previousScreenFilamentPosMin.CurrentPosition.x && x > previousScreenFilamentPosMax.CurrentPosition.x && y < previousScreenFilamentPosMin.CurrentPosition.y && y > previousScreenFilamentPosMax.CurrentPosition.y)
                        {
                            currentlyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(x, y)));
                        }
                    });
                });
                newlyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>(newlyProximalFilamentPositions.OrderBy((newlyProximalFilamentPosition) =>
                {
                    return -(new Universe.Filament.Position(transform.position).WorldPosition - newlyProximalFilamentPosition.WorldPosition).sqrMagnitude;
                }));
                #endregion
                
                #region Previously Proximal Filaments
                previouslyProximalFilamentPositions = new ConcurrentBag<Universe.Filament.Position>();

                Parallel.ForEach(currentUniverse.LoadedFilaments.Keys.Except(currentlyProximalFilamentPositions), (loadedFilamentPosition) =>
                {
                    if (loadedFilamentPosition.CurrentPosition.x < currentScreenFilamentPosMin.CurrentPosition.x || loadedFilamentPosition.CurrentPosition.x > currentScreenFilamentPosMax.CurrentPosition.x || loadedFilamentPosition.CurrentPosition.y < currentScreenFilamentPosMin.CurrentPosition.y || loadedFilamentPosition.CurrentPosition.y > currentScreenFilamentPosMax.CurrentPosition.y)
                    {
                        previouslyProximalFilamentPositions.Add(new Universe.Filament.Position(new Vector2Int(loadedFilamentPosition.CurrentPosition.x, loadedFilamentPosition.CurrentPosition.y)));
                    }
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

        private void UnloadPreviouslyProximalPositions()
        {
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
        }

        private void CancelInvalidatedProximalPositionLoadRequests()
        {
            // Region Chunk
            foreach (Universe.Region.Chunk.Position regionChunkLoadRequest in Universe.MapElementLoadingUtil.RegionChunkLoadRequests)
            {
                if (!currentlyProximalRegionChunkPositions.Contains(regionChunkLoadRequest))
                {
                    Universe.MapElementLoadingUtil.CancelRegionChunkLoadRequest(regionChunkLoadRequest);
                }
            }

            // Region
            foreach (Universe.Region.Position regionLoadRequest in Universe.MapElementLoadingUtil.RegionLoadRequests)
            {
                if (!currentlyProximalRegionPositions.Contains(regionLoadRequest))
                {
                    Universe.MapElementLoadingUtil.CancelRegionLoadRequest(regionLoadRequest);
                }
            }

            // Sector Chunk
            foreach (Universe.Sector.Chunk.Position sectorChunkLoadRequest in Universe.MapElementLoadingUtil.SectorChunkLoadRequests)
            {
                if (currentlyProximalSectorChunkPositions.Contains(sectorChunkLoadRequest))
                {
                    Universe.MapElementLoadingUtil.CancelSectorChunkLoadRequest(sectorChunkLoadRequest);
                }
            }

            // Sector
            foreach (Universe.Sector.Position sectorLoadRequest in Universe.MapElementLoadingUtil.SectorLoadRequests)
            {
                if (!currentlyProximalSectorPositions.Contains(sectorLoadRequest))
                {
                    Universe.MapElementLoadingUtil.CancelSectorLoadRequest(sectorLoadRequest);
                }
            }

            // Filament Chunk
            foreach (Universe.Filament.Chunk.Position filamentChunkLoadRequest in Universe.MapElementLoadingUtil.FilamentChunkLoadRequests)
            {
                if (currentlyProximalFilamentChunkPositions.Contains(filamentChunkLoadRequest))
                {
                    Universe.MapElementLoadingUtil.CancelFilamentChunkLoadRequest(filamentChunkLoadRequest);
                }
            }

            // Filament
            foreach (Universe.Filament.Position filamentLoadRequest in Universe.MapElementLoadingUtil.FilamentLoadRequests)
            {
                if (!currentlyProximalFilamentPositions.Contains(filamentLoadRequest))
                {
                    Universe.MapElementLoadingUtil.CancelFilamentLoadRequest(filamentLoadRequest);
                }
            }
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
        #endregion

        #region Coroutines
        private IEnumerator LoadNewlyProximalPositionsCoroutine()
        {
            while (true)
            {
                yield return StartCoroutine(LoadNewlyProximalFilamentPositionsCoroutine());
                yield return StartCoroutine(LoadNewlyProximalFilamentChunkPositionsCoroutine());
                yield return StartCoroutine(LoadNewlyProximalSectorPositionsCoroutine());
                yield return StartCoroutine(LoadNewlyProximalSectorChunkPositionsCoroutine());
                yield return StartCoroutine(LoadNewlyProximalRegionPositionsCoroutine());
                yield return StartCoroutine(LoadNewlyProximalRegionChunkPositionsCoroutine());

                yield return null;
            }
        }

        private IEnumerator LoadNewlyProximalFilamentPositionsCoroutine()
        {
            foreach (var newlyProximalFilamentPosition in newlyProximalFilamentPositions)
            {
                if (currentUniverse.IsFilamentGenerationRequested(newlyProximalFilamentPosition))
                {
                    continue;
                }
                else
                {
                    if (currentUniverse.IsFilamentLoaded(newlyProximalFilamentPosition))
                    {
                        continue;
                    }
                    else
                    {
                        if (currentUniverse.IsFilamentGenerated(newlyProximalFilamentPosition))
                        {
                            currentUniverse.LoadFilament(newlyProximalFilamentPosition);
                            yield return null;
                            continue;
                        }
                        else
                        {
                            currentUniverse.RequestGenerateFilament(newlyProximalFilamentPosition);
                            yield return null;
                        }
                    }
                }
            }
        }

        private IEnumerator LoadNewlyProximalFilamentChunkPositionsCoroutine()
        {
            foreach (var newlyProximalFilamentChunkPosition in newlyProximalFilamentChunkPositions)
            {
                if (currentUniverse.IsFilamentChunkGenerationRequested(newlyProximalFilamentChunkPosition))
                {
                    continue;
                }
                else
                {
                    if (currentUniverse.IsFilamentChunkLoaded(newlyProximalFilamentChunkPosition))
                    {
                        continue;
                    }
                    else
                    {
                        if (currentUniverse.IsFilamentChunkGenerated(newlyProximalFilamentChunkPosition))
                        {
                            currentUniverse.LoadFilamentChunk(newlyProximalFilamentChunkPosition);
                            yield return null;
                            continue;
                        }
                        else
                        {
                            currentUniverse.RequestGenerateFilamentChunk(newlyProximalFilamentChunkPosition);
                            yield return null;
                        }
                    }
                }
            }
        }

        private IEnumerator LoadNewlyProximalSectorPositionsCoroutine()
        {
            foreach (var newlyProximalSectorPosition in newlyProximalSectorPositions)
            {
                if (currentUniverse.IsSectorGenerationRequested(newlyProximalSectorPosition))
                {
                    continue;
                }
                else
                {
                    if (currentUniverse.IsSectorLoaded(newlyProximalSectorPosition))
                    {
                        continue;
                    }
                    else
                    {
                        if (currentUniverse.IsSectorGenerated(newlyProximalSectorPosition))
                        {
                            currentUniverse.LoadSector(newlyProximalSectorPosition);
                            yield return null;
                            continue;
                        }
                        else
                        {
                            currentUniverse.RequestGenerateSector(newlyProximalSectorPosition);
                            yield return null;
                        }
                    }
                }
            }
        }

        private IEnumerator LoadNewlyProximalSectorChunkPositionsCoroutine()
        {
            foreach (var newlyProximalSectorChunkPosition in newlyProximalSectorChunkPositions)
            {
                if (currentUniverse.IsSectorChunkGenerationRequested(newlyProximalSectorChunkPosition))
                {
                    continue;
                }
                else
                {
                    if (currentUniverse.IsSectorChunkLoaded(newlyProximalSectorChunkPosition))
                    {
                        continue;
                    }
                    else
                    {
                        if (currentUniverse.IsSectorChunkGenerated(newlyProximalSectorChunkPosition))
                        {
                            currentUniverse.LoadSectorChunk(newlyProximalSectorChunkPosition);
                            yield return null;
                            continue;
                        }
                        else
                        {
                            currentUniverse.RequestGenerateSectorChunk(newlyProximalSectorChunkPosition);
                            yield return null;
                        }
                    }
                }
            }
        }

        private IEnumerator LoadNewlyProximalRegionPositionsCoroutine()
        {
            foreach (var newlyProximalRegionPosition in newlyProximalRegionPositions)
            {
                if (currentUniverse.IsRegionGenerationRequested(newlyProximalRegionPosition))
                {
                    continue;
                }
                else
                {
                    if (currentUniverse.IsRegionLoaded(newlyProximalRegionPosition))
                    {
                        continue;
                    }
                    else
                    {
                        if (currentUniverse.IsRegionGenerated(newlyProximalRegionPosition))
                        {
                            currentUniverse.LoadRegion(newlyProximalRegionPosition);
                            yield return null;
                            continue;
                        }
                        else
                        {
                            currentUniverse.RequestGenerateRegion(newlyProximalRegionPosition);
                            yield return null;
                        }
                    }
                }
            }
        }

        private IEnumerator LoadNewlyProximalRegionChunkPositionsCoroutine()
        {
            foreach (var newlyProximalRegionChunkPosition in newlyProximalRegionChunkPositions)
            {
                if (currentUniverse.IsRegionChunkGenerationRequested(newlyProximalRegionChunkPosition))
                {
                    continue;
                }
                else
                {
                    if (currentUniverse.IsRegionChunkLoaded(newlyProximalRegionChunkPosition))
                    {
                        continue;
                    }
                    else
                    {
                        if (currentUniverse.IsRegionChunkGenerated(newlyProximalRegionChunkPosition))
                        {
                            currentUniverse.LoadRegionChunk(newlyProximalRegionChunkPosition);
                            yield return null;
                            continue;
                        }
                        else
                        {
                            currentUniverse.RequestGenerateRegionChunk(newlyProximalRegionChunkPosition);
                            yield return null;
                        }
                    }
                }
            }
        }
        #endregion
    }
}
