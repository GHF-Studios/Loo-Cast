using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.UI.Screen
{
    using LooCast.UI.Panel;
    using LooCast.Station;

    public class StationScreen : Screen
    {
        public PlayerStation PlayerStation
        {
            get
            {
                return playerStation;
            }

            set
            {
                if (value == null)
                {
                    
                }
                else
                {
                    
                }
                playerStation = value;
            }
        }

        [SerializeField] private StationHUBPanel stationHUBPanel;
        [SerializeField] private StationMarketPanel stationMarketPanel;
        [SerializeField] private StationMissionPanel stationMissionPanel;
        [SerializeField] private StationManufacturingPanel stationManufacturingPanel;
        [SerializeField] private StationUpgradesPanel stationUpgradesPanel;
        [SerializeField] private StationBlackmarketPanel stationBlackmarketPanel;
        private Panel activePanel;

        private PlayerStation playerStation;

        private void Start()
        {
            isInitiallyVisible = false;
            isHideable = true;
            Initialize();

            ShowStationHUB();
        }

        public override void Refresh()
        {
            if (PlayerStation == null)
            {
                SetVisibility(false);
                return;
            }
            if (enabled)
            {
                
            }
        }

        public void ShowStationHUB()
        {
            activePanel = stationHUBPanel;
            stationHUBPanel.enabled = true;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;

            Refresh();
        }

        public void ShowMarket()
        {
            activePanel = stationMarketPanel;
            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = true;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;

            Refresh();
        }

        public void ShowMissions()
        {
            activePanel = stationMissionPanel;
            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = true;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;

            Refresh();
        }

        public void ShowManufacturing()
        {
            activePanel = stationManufacturingPanel;
            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = true;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = false;

            Refresh();
        }

        public void ShowUpgrades()
        {
            activePanel = stationUpgradesPanel;
            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = true;
            stationBlackmarketPanel.enabled = false;

            Refresh();
        }

        public void ShowBlackmarket()
        {
            activePanel = stationBlackmarketPanel;
            stationHUBPanel.enabled = false;
            stationMarketPanel.enabled = false;
            stationMissionPanel.enabled = false;
            stationManufacturingPanel.enabled = false;
            stationUpgradesPanel.enabled = false;
            stationBlackmarketPanel.enabled = true;

            Refresh();
        }
    } 
}
