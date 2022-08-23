namespace LooCast.Item
{
    public class ItemSlot
    {
        public Item Item
        {
            get
            {
                return item;
            }

            set
            {
                item = value;
            }
        }
        private Item item;
    }
}