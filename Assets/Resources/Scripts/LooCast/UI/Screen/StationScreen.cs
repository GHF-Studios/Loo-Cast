using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Screen
{
    using LooCast.UI.Panel;
    using LooCast.Station;

    public class StationScreen : Screen
    {
        public AllyStation CurrentAllyStation
        {
            get
            {
                return currentAllyStation;
            }

            set
            {
                currentAllyStation = value;
            }
        }

        [SerializeField] private StationHUBPanel stationHUBPanel;
        [SerializeField] private StationMarketPanel stationMarketPanel;
        [SerializeField] private StationMissionPanel stationMissionPanel;
        [SerializeField] private StationManufacturingPanel stationManufacturingPanel;
        [SerializeField] private StationUpgradesPanel stationUpgradesPanel;
        [SerializeField] private StationBlackmarketPanel stationBlackmarketPanel;

        private AllyStation currentAllyStation;

        private void Start()
        {
            isInitiallyVisible = false;
            isHideable = true;
            Initialize();
        }

        public override void SetVisibility(bool show)
        {
            base.SetVisibility(show);
            if (show)
            {
                ShowMissions();
            }
        }

        public void ShowStationHUB()
        {
            if (CurrentAllyStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh StationHUB
            }

            stationHUBPanel.gameObject.SetActive(true);
            stationMarketPanel.gameObject.SetActive(false);
            stationMissionPanel.gameObject.SetActive(false);
            stationManufacturingPanel.gameObject.SetActive(false);
            stationUpgradesPanel.gameObject.SetActive(false);
            stationBlackmarketPanel.gameObject.SetActive(false);
        }

        public void ShowMarket()
        {
            if (CurrentAllyStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh StationHUB
            }

            stationHUBPanel.gameObject.SetActive(false);
            stationMarketPanel.gameObject.SetActive(true);
            stationMissionPanel.gameObject.SetActive(false);
            stationManufacturingPanel.gameObject.SetActive(false);
            stationUpgradesPanel.gameObject.SetActive(false);
            stationBlackmarketPanel.gameObject.SetActive(false);
        }

        public void ShowMissions()
        {
            if (CurrentAllyStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Missions
                stationMissionPanel.MissionProvider = CurrentAllyStation.MissionProvider;
            }
            
            stationHUBPanel.gameObject.SetActive(false);
            stationMarketPanel.gameObject.SetActive(false);
            stationMissionPanel.gameObject.SetActive(true);
            stationManufacturingPanel.gameObject.SetActive(false);
            stationUpgradesPanel.gameObject.SetActive(false);
            stationBlackmarketPanel.gameObject.SetActive(false);
        }

        public void ShowManufacturing()
        {
            if (CurrentAllyStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Manufacturing
            }

            stationHUBPanel.gameObject.SetActive(false);
            stationMarketPanel.gameObject.SetActive(false);
            stationMissionPanel.gameObject.SetActive(false);
            stationManufacturingPanel.gameObject.SetActive(true);
            stationUpgradesPanel.gameObject.SetActive(false);
            stationBlackmarketPanel.gameObject.SetActive(false);
        }

        public void ShowUpgrades()
        {
            if (CurrentAllyStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Upgrades
            }

            stationHUBPanel.gameObject.SetActive(false);
            stationMarketPanel.gameObject.SetActive(false);
            stationMissionPanel.gameObject.SetActive(false);
            stationManufacturingPanel.gameObject.SetActive(false);
            stationUpgradesPanel.gameObject.SetActive(true);
            stationBlackmarketPanel.gameObject.SetActive(false);
        }

        public void ShowBlackmarket()
        {
            if (CurrentAllyStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Blackmarket
            }

            stationHUBPanel.gameObject.SetActive(false);
            stationMarketPanel.gameObject.SetActive(false);
            stationMissionPanel.gameObject.SetActive(false);
            stationManufacturingPanel.gameObject.SetActive(false);
            stationUpgradesPanel.gameObject.SetActive(false);
            stationBlackmarketPanel.gameObject.SetActive(true);
        }
    } 
}
