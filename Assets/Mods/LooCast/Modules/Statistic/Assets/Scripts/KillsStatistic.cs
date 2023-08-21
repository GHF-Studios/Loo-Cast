using Steamworks;

namespace LooCast.Statistic
{
    public static class KillsStatistic
    {
        public static int Kills
        {
            get
            {
                return kills;
            }
            set
            {
                kills = value;
                if (SteamManager.Initialized)
                {
                    SteamUserStats.GetStat("highscore_kills", out int highscore_kills);
                    if (Kills > highscore_kills)
                    {
                        SteamUserStats.SetStat("highscore_kills", Kills);
                    }
                    if (Kills >= 420)
                    {
                        SteamUserStats.GetAchievement("The_Funny_Number", out bool achievementCompleted);
                        if (!achievementCompleted)
                        {
                            SteamUserStats.SetAchievement("The_Funny_Number");
                        }
                    }
                    SteamUserStats.StoreStats();
                }
            }
        }
        private static int kills = 0;
    }
}
