use crate::{VcmpFunctions, options::VcmpNetworkStatisticsQueryOption};

pub trait QueryNetworkStatistics {
    fn data_sent_per_second(&self, player_id: i32) -> f64;
    fn data_resent_per_second(&self, player_id: i32) -> f64;
    fn data_received_per_second(&self, player_id: i32) -> f64;
    fn data_discarded_per_second(&self, player_id: i32) -> f64;
    fn all_bytes_sent_per_second(&self, player_id: i32) -> f64;
    fn all_bytes_received_per_second(&self, player_id: i32) -> f64;

    fn data_sent_total(&self, player_id: i32) -> f64;
    fn data_resent_total(&self, player_id: i32) -> f64;
    fn data_received_total(&self, player_id: i32) -> f64;
    fn data_discarded_total(&self, player_id: i32) -> f64;
    fn all_bytes_sent_total(&self, player_id: i32) -> f64;
    fn all_bytes_received_total(&self, player_id: i32) -> f64;

    fn messages_waiting(&self, player_id: i32) -> f64;
    fn messages_resending(&self, player_id: i32) -> f64;
    fn bytes_resending(&self, player_id: i32) -> f64;

    fn packet_loss_per_second(&self, player_id: i32) -> f64;
    fn packet_loss_total(&self, player_id: i32) -> f64;
}

impl QueryNetworkStatistics for VcmpFunctions {
    fn data_sent_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::DataSentPerSecond,
        )
    }

    fn data_resent_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::DataResentPerSecond,
        )
    }

    fn data_received_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::DataReceivedPerSecond,
        )
    }

    fn data_discarded_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::DataDiscardedPerSecond,
        )
    }

    fn all_bytes_sent_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::AllBytesSentPerSecond,
        )
    }

    fn all_bytes_received_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::AllBytesReceivedPerSecond,
        )
    }

    fn data_sent_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(player_id, VcmpNetworkStatisticsQueryOption::DataSentTotal)
    }

    fn data_resent_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(player_id, VcmpNetworkStatisticsQueryOption::DataResentTotal)
    }

    fn data_received_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::DataReceivedTotal,
        )
    }

    fn data_discarded_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::DataDiscardedTotal,
        )
    }

    fn all_bytes_sent_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::AllBytesSentTotal,
        )
    }

    fn all_bytes_received_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::AllBytesReceivedTotal,
        )
    }

    fn messages_waiting(&self, player_id: i32) -> f64 {
        self.get_network_statistics(player_id, VcmpNetworkStatisticsQueryOption::MessagesWaiting)
    }

    fn messages_resending(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::MessagesResending,
        )
    }

    fn bytes_resending(&self, player_id: i32) -> f64 {
        self.get_network_statistics(player_id, VcmpNetworkStatisticsQueryOption::BytesResending)
    }

    fn packet_loss_per_second(&self, player_id: i32) -> f64 {
        self.get_network_statistics(
            player_id,
            VcmpNetworkStatisticsQueryOption::PacketLossPerSecond,
        )
    }

    fn packet_loss_total(&self, player_id: i32) -> f64 {
        self.get_network_statistics(player_id, VcmpNetworkStatisticsQueryOption::PacketLossTotal)
    }
}
