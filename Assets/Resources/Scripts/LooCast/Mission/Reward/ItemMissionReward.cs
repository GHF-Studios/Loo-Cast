namespace LooCast.Mission.Reward
{
    using LooCast.Item;
    using LooCast.Item.Data;

    public class ItemMissionReward : MissionReward
    {
        public ItemContainer ItemContainerRewardee { get; private set; }
        public ItemData RewardedItemData { get; private set; }

        public ItemMissionReward(ItemContainer itemContainerRewardee, ItemData rewardedItemData) : base()
        {
            ItemContainerRewardee = itemContainerRewardee;
            RewardedItemData = rewardedItemData;
        }

        public override void Reward()
        {
            Item rewardedItem = RewardedItemData.CreateItem();
            ItemContainerRewardee.AddItem(rewardedItem, out Item remainingItem);
            if (remainingItem != null)
            {
                throw new System.Exception("Cannot Reward Item to Item Container!");
            }
        }
    }
}