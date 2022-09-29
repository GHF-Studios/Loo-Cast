using System;
using UnityEngine;

namespace LooCast.Test
{
    using LooCast.Universe;
    
    public class UniverseGenerator : MonoBehaviour
    {
        [SerializeField] private Universe.GenerationSettings DEV_UNIVERSE_universeGenerationSettings;

        [SerializeField] private Vector2Int DEV_FILAMENT_filamentPosition;
        [SerializeField] private Vector2Int[] DEV_FILAMENT_filamentPositions;
        [SerializeField] private Vector2Int DEV_FILAMENT_filamentPositionRangeMin;
        [SerializeField] private Vector2Int DEV_FILAMENT_filamentPositionRangeMax;

        [SerializeField] private Vector2Int DEV_SECTOR_filamentPosition;
        [SerializeField] private Vector2Int DEV_SECTOR_sectorPosition;
        [SerializeField] private Vector2Int[] DEV_SECTOR_sectorPositions;
        [SerializeField] private Vector2Int DEV_SECTOR_sectorPositionRangeMin;
        [SerializeField] private Vector2Int DEV_SECTOR_sectorPositionRangeMax;

        [SerializeField] private Vector2Int DEV_REGION_sectorPosition;
        [SerializeField] private Vector2Int DEV_REGION_regionPosition;
        [SerializeField] private Vector2Int[] DEV_REGION_regionPositions;
        [SerializeField] private Vector2Int DEV_REGION_regionPositionRangeMin;
        [SerializeField] private Vector2Int DEV_REGION_regionPositionRangeMax;

        public bool DEV_UNIVERSE_showSection = false;
        public bool DEV_FILAMENT_showSection = false;
        public bool DEV_SECTOR_showSection = false;
        public bool DEV_REGION_showSection = false;

        private Vector2Int[] GetPositionsFromRange(Vector2Int rangeMin, Vector2Int rangeMax)
        {
            int arrayWidth = rangeMax.x - rangeMin.x + 1;
            int arrayHeight = rangeMax.y - rangeMin.y + 1;
            Vector2Int[] positions = new Vector2Int[arrayWidth * arrayHeight];
            for (int y = rangeMin.y; y <= rangeMax.y; y++)
            {
                for (int x = rangeMin.x; x <= rangeMax.x; x++)
                {
                    positions[(y - rangeMin.y) * arrayWidth + (x - rangeMin.x)] = new Vector2Int(x, y);
                }
            }
            return positions;
        }

        #region Universe
        public void GenerateUniverse()
        {
            Universe.GenerateUniverse(DEV_UNIVERSE_universeGenerationSettings);
        }

        public void SaveUniverse()
        {
            Universe.SaveUniverse();
        }

        public void LoadUniverse()
        {
            Universe.LoadUniverse();
        }

        public void UnloadUniverse()
        {
            Universe.UnloadUniverse();
        }

        public void DeleteUniverse()
        {
            Universe.DeleteUniverse();
        }
        #endregion

        #region Filaments

        #region Generation
        public void GenerateFilament()
        {
            Universe.Instance.GenerateFilament(DEV_FILAMENT_filamentPosition);
        }

        public void GenerateFilaments()
        {
            Universe.Instance.GenerateFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void GenerateFilamentRange()
        {
            Universe.Instance.GenerateFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }
        #endregion

        #region Saving
        public void SaveFilament()
        {
            Universe.Instance.SaveFilament(DEV_FILAMENT_filamentPosition);
        }

        public void SaveFilaments()
        {
            Universe.Instance.SaveFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void SaveFilamentRange()
        {
            Universe.Instance.SaveFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }

        public void SaveAllFilaments()
        {
            Universe.Instance.SaveFilaments();
        }
        #endregion

        #region Loading
        public void LoadFilament()
        {
            Universe.Instance.LoadFilament(DEV_FILAMENT_filamentPosition);
        }

        public void LoadFilaments()
        {
            Universe.Instance.LoadFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void LoadFilamentRange()
        {
            Universe.Instance.LoadFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }

        public void UnloadFilament()
        {
            Universe.Instance.UnloadFilament(DEV_FILAMENT_filamentPosition);
        }

        public void UnloadFilaments()
        {
            Universe.Instance.UnloadFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void UnloadFilamentRange()
        {
            Universe.Instance.UnloadFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }

        public void UnloadAllFilaments()
        {
            Universe.Instance.UnloadFilaments();
        }
        #endregion

        #region Deletion
        public void DeleteFilament()
        {
            Universe.Instance.DeleteFilament(DEV_FILAMENT_filamentPosition);
        }

        public void DeleteFilaments()
        {
            Universe.Instance.DeleteFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void DeleteFilamentRange()
        {
            Universe.Instance.DeleteFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }

        public void DeleteAllFilaments()
        {
            Universe.Instance.DeleteFilaments();
        }
        #endregion

        #region Spawning
        public void SpawnFilament()
        {
            Universe.Instance.SpawnFilament(DEV_FILAMENT_filamentPosition);
        }

        public void SpawnFilaments()
        {
            Universe.Instance.SpawnFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void SpawnFilamentRange()
        {
            Universe.Instance.SpawnFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }

        public void DespawnFilament()
        {
            Universe.Instance.DespawnFilament(DEV_FILAMENT_filamentPosition);
        }

        public void DespawnFilaments()
        {
            Universe.Instance.DespawnFilaments(DEV_FILAMENT_filamentPositions);
        }

        public void DespawnFilamentRange()
        {
            Universe.Instance.DespawnFilaments(GetPositionsFromRange(DEV_FILAMENT_filamentPositionRangeMin, DEV_FILAMENT_filamentPositionRangeMax));
        }
        #endregion

        #endregion

        #region Sector

        #region Generation
        public void GenerateSector()
        {
            Universe.Instance.GenerateSector(DEV_SECTOR_filamentPosition, DEV_SECTOR_sectorPosition);
        }

        public void GenerateSectors()
        {
            Universe.Instance.GenerateSectors(DEV_SECTOR_filamentPosition, DEV_SECTOR_sectorPositions);
        }

        public void GenerateSectorRange()
        {
            Universe.Instance.GenerateSectors(DEV_SECTOR_filamentPosition, GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }
        #endregion

        #region Saving
        public void SaveSector()
        {
            Universe.Instance.SaveSector(DEV_SECTOR_sectorPosition);
        }

        public void SaveSectors()
        {
            Universe.Instance.SaveSectors(DEV_SECTOR_sectorPositions);
        }

        public void SaveSectorRange()
        {
            Universe.Instance.SaveSectors(GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }

        public void SaveAllSectors()
        {
            Universe.Instance.SaveSectors();
        }
        #endregion

        #region Loading
        public void LoadSector()
        {
            Universe.Instance.LoadSector(DEV_SECTOR_sectorPosition);
        }

        public void LoadSectors()
        {
            Universe.Instance.LoadSectors(DEV_SECTOR_sectorPositions);
        }

        public void LoadSectorRange()
        {
            Universe.Instance.LoadSectors(GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }

        public void UnloadSector()
        {
            Universe.Instance.UnloadSector(DEV_SECTOR_sectorPosition);
        }

        public void UnloadSectors()
        {
            Universe.Instance.UnloadSectors(DEV_SECTOR_sectorPositions);
        }

        public void UnloadSectorRange()
        {
            Universe.Instance.UnloadSectors(GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }

        public void UnloadAllSectors()
        {
            Universe.Instance.UnloadSectors();
        }
        #endregion

        #region Deletion
        public void DeleteSector()
        {
            Universe.Instance.DeleteSector(DEV_SECTOR_sectorPosition);
        }

        public void DeleteSectors()
        {
            Universe.Instance.DeleteSectors(DEV_SECTOR_sectorPositions);
        }

        public void DeleteSectorRange()
        {
            Universe.Instance.DeleteSectors(GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }

        public void DeleteAllSectors()
        {
            Universe.Instance.DeleteSectors();
        }
        #endregion

        #region Spawning
        public void SpawnSector()
        {
            Universe.Instance.SpawnSector(DEV_SECTOR_sectorPosition);
        }

        public void SpawnSectors()
        {
            Universe.Instance.SpawnSectors(DEV_SECTOR_sectorPositions);
        }

        public void SpawnSectorRange()
        {
            Universe.Instance.SpawnSectors(GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }

        public void DespawnSector()
        {
            Universe.Instance.DespawnSector(DEV_SECTOR_sectorPosition);
        }

        public void DespawnSectors()
        {
            Universe.Instance.DespawnSectors(DEV_SECTOR_sectorPositions);
        }

        public void DespawnSectorRange()
        {
            Universe.Instance.DespawnSectors(GetPositionsFromRange(DEV_SECTOR_sectorPositionRangeMin, DEV_SECTOR_sectorPositionRangeMax));
        }
        #endregion

        #endregion

        #region Region

        #region Generation
        public void GenerateRegion()
        {
            Universe.Instance.GenerateRegion(DEV_REGION_sectorPosition, DEV_REGION_regionPosition);
        }

        public void GenerateRegions()
        {
            Universe.Instance.GenerateRegions(DEV_REGION_sectorPosition, DEV_REGION_regionPositions);
        }

        public void GenerateRegionRange()
        {
            Universe.Instance.GenerateRegions(DEV_REGION_sectorPosition, GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }
        #endregion

        #region Saving
        public void SaveRegion()
        {
            Universe.Instance.SaveRegion(DEV_REGION_regionPosition);
        }

        public void SaveRegions()
        {
            Universe.Instance.SaveRegions(DEV_REGION_regionPositions);
        }

        public void SaveRegionRange()
        {
            Universe.Instance.SaveRegions(GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }

        public void SaveAllRegions()
        {
            Universe.Instance.SaveRegions();
        }
        #endregion

        #region Loading
        public void LoadRegion()
        {
            Universe.Instance.LoadRegion(DEV_REGION_regionPosition);
        }

        public void LoadRegions()
        {
            Universe.Instance.LoadRegions(DEV_REGION_regionPositions);
        }

        public void LoadRegionRange()
        {
            Universe.Instance.LoadRegions(GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }

        public void UnloadRegion()
        {
            Universe.Instance.UnloadRegion(DEV_REGION_regionPosition);
        }

        public void UnloadRegions()
        {
            Universe.Instance.UnloadRegions(DEV_REGION_regionPositions);
        }

        public void UnloadRegionRange()
        {
            Universe.Instance.UnloadRegions(GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }

        public void UnloadAllRegions()
        {
            Universe.Instance.UnloadRegions();
        }
        #endregion

        #region Deletion
        public void DeleteRegion()
        {
            Universe.Instance.DeleteRegion(DEV_REGION_regionPosition);
        }

        public void DeleteRegions()
        {
            Universe.Instance.DeleteRegions(DEV_REGION_regionPositions);
        }

        public void DeleteRegionRange()
        {
            Universe.Instance.DeleteRegions(GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }

        public void DeleteAllRegions()
        {
            Universe.Instance.DeleteRegions();
        }
        #endregion

        #region Spawning
        public void SpawnRegion()
        {
            Universe.Instance.SpawnRegion(DEV_REGION_regionPosition);
        }

        public void SpawnRegions()
        {
            Universe.Instance.SpawnRegions(DEV_REGION_regionPositions);
        }

        public void SpawnRegionRange()
        {
            Universe.Instance.SpawnRegions(GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }

        public void DespawnRegion()
        {
            Universe.Instance.DespawnRegion(DEV_REGION_regionPosition);
        }

        public void DespawnRegions()
        {
            Universe.Instance.DespawnRegions(DEV_REGION_regionPositions);
        }

        public void DespawnRegionRange()
        {
            Universe.Instance.DespawnRegions(GetPositionsFromRange(DEV_REGION_regionPositionRangeMin, DEV_REGION_regionPositionRangeMax));
        }
        #endregion

        #endregion
    }
}