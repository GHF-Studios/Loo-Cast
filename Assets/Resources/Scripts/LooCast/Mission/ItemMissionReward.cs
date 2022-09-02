namespace LooCast.Mission
{
    using LooCast.Item;

    public class ItemMissionReward : MissionReward
    {
        public ItemContainer<Item> ItemContainerRewardee { get; private set; }
        public Item RewardedItem { get; private set; }

        public ItemMissionReward(ItemContainer<Item> itemContainerRewardee, Item rewardedItem) : base()
        {
            ItemContainerRewardee = itemContainerRewardee;
            RewardedItem = rewardedItem;
        }

        public override void Reward()
        {
            ItemContainerRewardee.AddItem(RewardedItem, out Item remainingItem);
            if (remainingItem != null)
            {
                throw new System.Exception("Cannot Reward Item to Item Container!");
            }
        }
    }
}