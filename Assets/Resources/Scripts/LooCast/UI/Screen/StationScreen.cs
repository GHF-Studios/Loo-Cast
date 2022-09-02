using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Screen
{
    using LooCast.UI.Panel;
    using LooCast.Station;

    public class StationScreen : Screen
    {
        public PlayerStation CurrentPlayerStation
        {
            get
            {
                return currentPlayerStation;
            }

            set
            {
                currentPlayerStation = value;
            }
        }

        [SerializeField] private StationHUBPanel stationHUBPanel;
        [SerializeField] private StationMarketPanel stationMarketPanel;
        [SerializeField] private StationMissionPanel stationMissionPanel;
        [SerializeField] private StationManufacturingPanel stationManufacturingPanel;
        [SerializeField] private StationUpgradesPanel stationUpgradesPanel;
        [SerializeField] private StationBlackmarketPanel stationBlackmarketPanel;

        private PlayerStation currentPlayerStation;

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
            if (CurrentPlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh StationHUB
            }

            stationHUBPanel.enabled = true;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;
        }

        public void ShowMarket()
        {
            if (CurrentPlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh StationHUB
            }

            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = true;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;
        }

        public void ShowMissions()
        {
            if (CurrentPlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Missions
                stationMissionPanel.MissionProvider = CurrentPlayerStation.MissionProvider;
            }
            
            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = true;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;
        }

        public void ShowManufacturing()
        {
            if (CurrentPlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Manufacturing
            }

            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = true;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;
        }

        public void ShowUpgrades()
        {
            if (CurrentPlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Upgrades
            }

            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = true;
            stationBlackmarketPanel.enabled = false;
        }

        public void ShowBlackmarket()
        {
            if (CurrentPlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                //Refresh Blackmarket
            }

            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = true;
        }
    } 
}
